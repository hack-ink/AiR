// crates.io
use eframe::egui::*;
// self
use super::Panel;
use crate::air::AiRContext;

#[derive(Debug, Default)]
pub struct Tabs {
	pub focused_panel: Panel,
}
impl Tabs {
	pub fn draw(&mut self, ctx: &mut AiRContext, ui: &mut Ui, y: f32) {
		ui.horizontal(|ui| {
			let focused_panel = &mut self.focused_panel;

			if let Some(fp) = ctx.state.ui.focused_panel.try_read() {
				*focused_panel = *fp;
			}

			ui.set_height(y);
			if ui.selectable_value(focused_panel, Panel::Chat, Panel::Chat.name()).changed() {
				*ctx.state.ui.focused_panel.write() = *focused_panel;
			}

			ui.separator();
			if ui.selectable_value(focused_panel, Panel::Setting, Panel::Setting.name()).changed() {
				*ctx.state.ui.focused_panel.write() = *focused_panel;
			}
		});
	}
}
