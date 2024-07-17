// crates.io
use eframe::egui::*;
// self
use super::Panel;
use crate::{air::AiRContext, component::function::Function, widget};

#[derive(Debug, Default)]
pub struct Tabs {
	pub activated_function: Panel,
}
impl Tabs {
	pub fn draw(&mut self, ctx: &mut AiRContext, ui: &mut Ui, y: f32) {
		ui.horizontal(|ui| {
			let activated_function = &mut self.activated_function;

			ui.set_height(y);
			ui.vertical(|ui| {
				ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
					if let Some(fp) = ctx.state.ui.activated_function.try_read() {
						*activated_function = *fp;
					}

					if ui
						.selectable_value(activated_function, Panel::Chat, Panel::Chat.name())
						.changed()
					{
						*ctx.state.ui.activated_function.write() = *activated_function;
					}

					ui.separator();
					if ui
						.selectable_value(activated_function, Panel::Setting, Panel::Setting.name())
						.changed()
					{
						*ctx.state.ui.activated_function.write() = *activated_function;
					}
				});
			});

			if matches!(activated_function, Panel::Chat) {
				ui.vertical(|ui| {
					ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
						let active_func = &mut ctx.components.setting.chat.activated_function;

						if let Some(af) = ctx.state.chat.active_func.try_read() {
							*active_func = *af;
						}

						if ui.add(widget::combo_box("Activated Function", active_func)).changed() {
							*ctx.state.chat.active_func.write() = *active_func;
						}
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
