// crates.io
use eframe::egui::*;
// self
use super::Panel;
use crate::{air::AiRContext, component::function::Function, widget};

#[derive(Debug, Default)]
pub struct Tabs {
	pub focused_panel: Panel,
}
impl Tabs {
	pub fn draw(&mut self, ctx: &mut AiRContext, ui: &mut Ui, y: f32) {
		ui.horizontal(|ui| {
			let focused_panel = &mut self.focused_panel;

			ui.set_height(y);
			ui.vertical(|ui| {
				ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
					// TODO: fn try_update.
					if let Some(fp) = ctx.state.ui.focused_panel.try_read() {
						*focused_panel = *fp;
					}

					// TODO: fn set.
					if ui.selectable_value(focused_panel, Panel::Chat, Panel::Chat.name()).changed()
					{
						*ctx.state.ui.focused_panel.write() = *focused_panel;
					}

					ui.separator();
					if ui
						.selectable_value(focused_panel, Panel::Setting, Panel::Setting.name())
						.changed()
					{
						*ctx.state.ui.focused_panel.write() = *focused_panel;
					}
				});
			});

			if matches!(focused_panel, Panel::Chat) {
				ui.vertical(|ui| {
					ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
						let activated_function =
							&mut ctx.components.setting.chat.activated_function;

						if let Some(af) = ctx.state.chat.activated_function.try_read() {
							*activated_function = *af;
						}

						if ui
							.add(widget::combo_box("Activated Function", activated_function))
							.changed()
						{
							*ctx.state.chat.activated_function.write() = *activated_function;
						}
						// TODO: fn is_translate.
						if matches!(
							ctx.components.setting.chat.activated_function,
							Function::Translate
						) {
							let mut chat_need_reload = false;

							chat_need_reload |= ui
								.add(widget::combo_box(
									"Language A",
									&mut ctx.components.setting.chat.translation.a,
								))
								.changed();

							ui.label("←→");

							chat_need_reload |= ui
								.add(widget::combo_box(
									"Language B",
									&mut ctx.components.setting.chat.translation.b,
								))
								.changed();

							if chat_need_reload {
								ctx.services.chat.renew(
									&ctx.components.setting.ai,
									&ctx.components.setting.chat,
								);
							}
						}
					});
				});
			}
		});
	}
}
