mod panel;
use panel::{Chat, Panel, Setting};

mod util;

// crates.io
use eframe::egui::*;
// self
use crate::air::AiRContext;

trait UiT {
	fn draw(&mut self, ui: &mut Ui, ctx: &mut AiRContext);
}

#[derive(Debug, Default)]
pub struct Uis {
	pub focused_panel: Panel,
	pub chat: Chat,
	pub setting: Setting,
}
impl Uis {
	pub fn new() -> Self {
		Default::default()
	}

	pub fn draw(&mut self, mut ctx: AiRContext) {
		CentralPanel::default()
			// FIXME: it looks like there some invalid cache.
			.frame(util::transparent_frame(ctx.egui_ctx))
			.show(ctx.egui_ctx, |ui| {
				ui.horizontal(|ui| {
					ui.selectable_value(&mut self.focused_panel, Panel::Chat, Panel::Chat.name());
					ui.separator();
					ui.selectable_value(
						&mut self.focused_panel,
						Panel::Setting,
						Panel::Setting.name(),
					);
					ui.separator();
				});
				ui.separator();

				match self.focused_panel {
					Panel::Chat => self.chat.draw(ui, &mut ctx),
					Panel::Setting => self.setting.draw(ui, &mut ctx),
				}
			});
	}
}
