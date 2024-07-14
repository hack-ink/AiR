// crates.io
use eframe::egui::*;
// self
use super::Panel;

#[derive(Debug, Default)]
pub struct Tabs {
	pub focused_tab: Panel,
}
impl Tabs {
	pub fn draw(&mut self, ui: &mut Ui, height: f32) {
		ui.horizontal(|ui| {
			ui.set_height(height);
			ui.selectable_value(&mut self.focused_tab, Panel::Chat, Panel::Chat.name());
			ui.separator();
			ui.selectable_value(&mut self.focused_tab, Panel::Setting, Panel::Setting.name());
		});
	}
}
