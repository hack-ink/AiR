mod panel;
use panel::{Chat, Panel, Setting};

// crates.io
use eframe::egui::*;
// self
use crate::air::AiRContext;

trait UiT {
	fn draw(&mut self, ui: &mut Ui, ctx: &mut AiRContext);
}

#[derive(Debug, Default)]
pub struct Uis {
	focused_panel: Panel,
	chat: Chat,
	setting: Setting,
}
impl Uis {
	pub fn new() -> Self {
		Default::default()
	}

	pub fn draw(&mut self, mut ctx: AiRContext) {
		CentralPanel::default()
			// TODO:? transparent window.
			// FIXME: it looks like there some invalid cache.
			// .frame(util::transparent_frame(ctx.egui_ctx))
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

pub fn set_fonts(ctx: &Context) {
	let mut fonts = FontDefinitions::default();

	// Cascadia Code.
	fonts.font_data.insert(
		"Cascadia Code".into(),
		FontData::from_static(include_bytes!("../asset/CascadiaCode.ttf")),
	);
	fonts.families.entry(FontFamily::Proportional).or_default().insert(0, "Cascadia Code".into());
	fonts.families.entry(FontFamily::Monospace).or_default().insert(0, "Cascadia Code".into());
	// NotoSerifSC.
	fonts.font_data.insert(
		"NotoSerifSC".into(),
		FontData::from_static(include_bytes!("../asset/NotoSerifSC-VariableFont_wght.ttf")),
	);
	fonts.families.entry(FontFamily::Proportional).or_default().insert(1, "NotoSerifSC".into());
	fonts.families.entry(FontFamily::Monospace).or_default().insert(1, "NotoSerifSC".into());

	ctx.set_fonts(fonts);
}

pub fn set_font_size(ctx: &Context, font_size: f32) {
	ctx.style_mut(|s| {
		s.text_styles.values_mut().for_each(|s| s.size = font_size);
	});
}
