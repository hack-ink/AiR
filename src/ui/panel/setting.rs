// std
use std::sync::atomic::Ordering;
// crates.io
use eframe::egui::*;
// self
use super::super::UiT;
use crate::{air::AiRContext, widget};

#[derive(Debug, Default)]
pub struct Setting {
	api_key: ApiKeyWidget,
}
impl Setting {
	fn set_font_sizes(&self, ctx: &AiRContext) {
		ctx.egui_ctx.style_mut(|s| {
			s.text_styles
				.values_mut()
				.for_each(|s| s.size = ctx.components.setting.general.font_size);
		});
	}
}
impl UiT for Setting {
	fn draw(&mut self, ui: &mut Ui, ctx: &mut AiRContext) {
		ui.collapsing("General", |ui| {
			Grid::new("General").num_columns(2).striped(true).show(ui, |ui| {
				ui.label("Font Size");
				ui.horizontal(|ui| {
					ui.spacing_mut().slider_width = ui.available_width() - 56.;

					if ui
						.add(
							Slider::new(&mut ctx.components.setting.general.font_size, 9_f32..=16.)
								.step_by(1.)
								.fixed_decimals(0),
						)
						.changed()
					{
						self.set_font_sizes(ctx);
					}
				});
				ui.end_row();

				ui.label("Hide on Lost Focus");
				if ui
					.add(widget::toggle(&mut ctx.components.setting.general.hide_on_lost_focus))
					.changed()
				{
					ctx.state.general.hide_on_lost_focus.store(
						ctx.components.setting.general.hide_on_lost_focus,
						Ordering::Relaxed,
					);
				};
				ui.end_row();

				ui.add(widget::combo_box(
					"Active Function",
					&mut ctx.components.setting.general.active_func,
				));
				ui.end_row();
			});
		});

		ui.collapsing("AI", |ui| {
			let mut changed = false;

			Grid::new("AI").num_columns(2).striped(true).show(ui, |ui| {
				ui.label("API Base");
				let size = ui
					.horizontal(|ui| {
						let mut size = ui.available_size();

						size.x -= 56.;

						changed |= ui
							.add_sized(
								size,
								TextEdit::singleline(&mut ctx.components.setting.ai.api_base),
							)
							.changed();

						size
					})
					.inner;
				ui.end_row();

				ui.label("API Key");
				ui.horizontal(|ui| {
					changed |= ui
						.add_sized(
							size,
							TextEdit::singleline(&mut ctx.components.setting.ai.api_key)
								.password(self.api_key.visibility),
						)
						.changed();

					if ui.button(&self.api_key.label).clicked() {
						self.api_key.clicked();
					}
				});
				ui.end_row();

				// TODO: we might not need to renew the client if only the model changed.
				changed |= ui
					.add(widget::combo_box("Model", &mut ctx.components.setting.ai.model))
					.changed();
				ui.end_row();

				// TODO: we might not need to renew the client if only the temperature changed.
				ui.label("Temperature");
				ui.spacing_mut().slider_width = size.x;
				changed |= ui
					.add(
						Slider::new(&mut ctx.components.setting.ai.temperature, 0_f32..=2.)
							.fixed_decimals(1)
							.step_by(0.1),
					)
					.changed();
				ui.end_row();
			});

			if changed {
				ctx.services.chat.renew(&ctx.components.setting);
			}
		});

		// TODO: [`crate::component::setting::Chat`].
		ui.collapsing("Translation", |ui| {
			Grid::new("Translation").num_columns(2).striped(true).show(ui, |ui| {
				// TODO: A and B should be mutually exclusive.
				ui.add(widget::combo_box("A", &mut ctx.components.setting.chat.translation.a));
				ui.end_row();

				ui.add(widget::combo_box("B", &mut ctx.components.setting.chat.translation.b));
				ui.end_row();
			});
		});
	}
}

#[derive(Debug)]
pub struct ApiKeyWidget {
	pub label: String,
	pub visibility: bool,
}
impl ApiKeyWidget {
	pub fn clicked(&mut self) {
		self.label = match self.label.as_str() {
			"show" => "hide".into(),
			"hide" => "show".into(),
			_ => unreachable!(),
		};
		self.visibility = !self.visibility;
	}
}
impl Default for ApiKeyWidget {
	fn default() -> Self {
		Self { label: "show".into(), visibility: true }
	}
}
