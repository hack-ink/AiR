// std
use std::sync::atomic::Ordering;
// crates.io
use eframe::egui::{self, *};
// self
use super::super::UiT;
use crate::{
	air::AiRContext,
	component::{function::Function, openai::Model, setting::Language},
};

#[derive(Debug, Default)]
pub struct Setting {
	pub api_key: ApiKeyWidget,
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
				if ui.add(toggle(&mut ctx.components.setting.general.hide_on_lost_focus)).changed()
				{
					ctx.state.general.hide_on_lost_focus.store(
						ctx.components.setting.general.hide_on_lost_focus,
						Ordering::Relaxed,
					);
				};
				ui.end_row();

				ui.label("Active Function");
				ComboBox::from_id_source("Active Function")
					.selected_text(&ctx.components.setting.general.active_func)
					.show_ui(ui, |ui| {
						Function::basic_all().iter().for_each(|f| {
							ui.selectable_value(
								&mut ctx.components.setting.general.active_func,
								f.to_owned(),
								f,
							);
						});
					});
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
				ui.label("Model");
				ComboBox::from_id_source("Model")
					.selected_text(&ctx.components.setting.ai.model)
					.show_ui(ui, |ui| {
						Model::all().iter().for_each(|m| {
							changed |= ui
								.selectable_value(
									&mut ctx.components.setting.ai.model,
									m.to_owned(),
									m,
								)
								.changed();
						});
					});
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
				ui.label("A");
				ComboBox::from_id_source("A")
					.selected_text(&ctx.components.setting.chat.translation.a)
					.show_ui(ui, |ui| {
						Language::all().iter().for_each(|l| {
							ui.selectable_value(
								&mut ctx.components.setting.chat.translation.a,
								l.to_owned(),
								l,
							);
						});
					});
				ui.end_row();

				ui.label("B");
				ComboBox::from_id_source("B")
					.selected_text(&ctx.components.setting.chat.translation.b)
					.show_ui(ui, |ui| {
						Language::all().iter().for_each(|l| {
							ui.selectable_value(
								&mut ctx.components.setting.chat.translation.b,
								l.to_owned(),
								l,
							);
						});
					});
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

// https://github.com/emilk/egui/blob/aa96b257460a07b30489d104fae08d095a9e3a4e/crates/egui_demo_lib/src/demo/toggle_switch.rs#L109.
fn toggle(on: &mut bool) -> impl Widget + '_ {
	fn toggle_ui(ui: &mut Ui, on: &mut bool) -> Response {
		let desired_size = ui.spacing().interact_size.y * vec2(2.0, 1.0);
		let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());

		if response.clicked() {
			*on = !*on;

			response.mark_changed();
		}
		if ui.is_rect_visible(rect) {
			let how_on = ui.ctx().animate_bool_responsive(response.id, *on);
			let visuals = ui.style().interact_selectable(&response, *on);
			let rect = rect.expand(visuals.expansion);
			let radius = 0.5 * rect.height();

			ui.painter().rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);

			let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
			let center = egui::pos2(circle_x, rect.center().y);

			ui.painter().circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
		}

		response
	}

	move |ui: &mut Ui| toggle_ui(ui, on)
}
