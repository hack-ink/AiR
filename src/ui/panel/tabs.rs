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
			let focused_p = &mut self.focused_panel;

			ui.set_height(y);
			ui.vertical(|ui| {
				ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
					ctx.state.ui.focused_panel.try_sync_to(focused_p);

					let chat_p = ui.selectable_value(focused_p, Panel::Chat, Panel::Chat.name());

					ui.separator();

					let setting_p =
						ui.selectable_value(focused_p, Panel::Setting, Panel::Setting.name());

					ctx.state.ui.focused_panel.sync_on_change(chat_p, *focused_p);
					ctx.state.ui.focused_panel.sync_on_change(setting_p, *focused_p);
				});
			});

			if matches!(focused_p, Panel::Chat) {
				ui.vertical(|ui| {
					ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
						let acted_fn = &mut ctx.components.setting.chat.activated_function;

						ctx.state.chat.activated_function.try_sync_to(acted_fn);

						let mut reload_chat = false;
						let acted_fn_cb = ui.add(widget::combo_box("Activated Function", acted_fn));

						// TODO: fn is_translate.
						if matches!(acted_fn, Function::Translate) {
							reload_chat |= ui
								.add(widget::combo_box(
									"Language A",
									&mut ctx.components.setting.chat.translate.a,
								))
								.changed();

							ui.label("←→");

							reload_chat |= ui
								.add(widget::combo_box(
									"Language B",
									&mut ctx.components.setting.chat.translate.b,
								))
								.changed();
						}

						ctx.state.chat.activated_function.sync_on_change(acted_fn_cb, *acted_fn);

						if reload_chat {
							ctx.services
								.chat
								.renew(&ctx.components.setting.ai, &ctx.components.setting.chat);
						}
					});
				});
			}
		});
	}
}
