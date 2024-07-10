// std
use std::sync::atomic::Ordering;
// crates.io
use eframe::egui::*;
// self
use super::super::UiT;
use crate::{
	air::AiRContext,
	widget::{self, HotkeyListener},
};

#[derive(Debug, Default)]
pub struct Setting {
	api_key: ApiKeyWidget,
	hotkey_listeners: [HotkeyListener; 4],
}
impl UiT for Setting {
	fn draw(&mut self, ui: &mut Ui, ctx: &mut AiRContext) {
		ScrollArea::vertical().id_source("Setting").auto_shrink(false).show(ui, |ui| {
			let margin = 36. + ctx.components.setting.general.font_size * 2.;

			ui.collapsing("General", |ui| {
				Grid::new("General").num_columns(2).show(ui, |ui| {
					ui.label("Font Size");
					ui.horizontal(|ui| {
						ui.spacing_mut().slider_width = ui.available_width() - margin;

						if ui
							.add(
								Slider::new(
									&mut ctx.components.setting.general.font_size,
									9_f32..=16.,
								)
								.step_by(1.)
								.fixed_decimals(0),
							)
							.changed()
						{
							super::super::set_font_size(
								ctx.egui_ctx,
								ctx.components.setting.general.font_size,
							);
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

					// TODO?: separate functions into different panels; then we won't need this.
					ui.add(widget::combo_box(
						"Active Function",
						&mut ctx.components.setting.general.active_func,
					));
					ui.end_row();
				});
			});

			ui.collapsing("AI", |ui| {
				Grid::new("AI").num_columns(2).show(ui, |ui| {
					let mut changed = false;

					ui.label("API Base");
					// The available size only works after there is an existing element.
					let mut size = ui.available_size();
					size.x -= margin;
					ui.horizontal(|ui| {
						changed |= ui
							.add_sized(
								size,
								TextEdit::singleline(&mut ctx.components.setting.ai.api_base),
							)
							.changed();
					});
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

					changed |= ui
						.add(widget::combo_box("Model", &mut ctx.components.setting.ai.model))
						.changed();
					ui.end_row();

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

					if changed {
						ctx.services
							.chat
							.renew(&ctx.components.setting.ai, &ctx.components.setting.chat);
					}
				});
			});

			ui.collapsing("Translation", |ui| {
				Grid::new("Translation").num_columns(2).show(ui, |ui| {
					// TODO: A and B should be mutually exclusive.
					for (l, c) in [
						("Language A", &mut ctx.components.setting.chat.translation.a),
						("Language B", &mut ctx.components.setting.chat.translation.b),
					] {
						ui.add(widget::combo_box(l, c));
						ui.end_row();
					}
					ui.end_row();
				});
			});

			ui.collapsing("Hotkey", |ui| {
				Grid::new("Hotkey").num_columns(2).show(ui, |ui| {
					if self
						.hotkey_listeners
						.iter_mut()
						.zip(
							[
								("Rewrite", &mut ctx.components.setting.hotkeys.rewrite),
								(
									"Rewrite Directly",
									&mut ctx.components.setting.hotkeys.rewrite_directly,
								),
								("Translate", &mut ctx.components.setting.hotkeys.translate),
								(
									"Translate Directly",
									&mut ctx.components.setting.hotkeys.translate_directly,
								),
							]
							.iter_mut(),
						)
						.fold(false, |mut changed, (kl, (l, hk))| {
							changed |= kl.listen(ui, l, hk);

							ui.end_row();

							changed
						}) {
						ctx.services.hotkey.renew(&ctx.components.setting.hotkeys);
					}
				});
			});

			ui.collapsing("Development", |ui| {
				Grid::new("Development").num_columns(2).show(ui, |ui| {
					if ui
						.add(widget::combo_box(
							"Log Level",
							&mut ctx.components.setting.development.log_level,
						))
						.changed()
					{
						ctx.state
							.development
							.reload_log_filter(
								ctx.components.setting.development.log_level.clone().into(),
							)
							.expect("reload must succeed");
					}
					ui.end_row();
				});
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
