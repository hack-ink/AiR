mod panel;
use panel::*;

// crates.io
use eframe::egui::*;
// self
use crate::air::AiRContext;

#[derive(Debug, Default)]
pub struct Uis {
	tabs: Tabs,
	chat: Chat,
	setting: Setting,
	status: Status,
}
impl Uis {
	pub fn new() -> Self {
		Default::default()
	}

	pub fn draw(&mut self, mut ctx: AiRContext) {
		let bar_h = ctx.components.setting.general.font_size + 10.;

		// Tabs.
		TopBottomPanel::top("Top Panel").show(ctx.egui_ctx, |ui| self.tabs.draw(ui, bar_h));
		// Main body.
		CentralPanel::default().show(ctx.egui_ctx, |ui| match self.tabs.focused_tab {
			Panel::Chat => {
				self.chat.draw(&mut ctx, ui, bar_h);

				// Status bar.
				TopBottomPanel::bottom("Bottom Panel")
					.show(ctx.egui_ctx, |ui| self.status.draw(&mut ctx, ui, bar_h, &self.chat));
			},
			Panel::Setting => self.setting.draw(&mut ctx, ui),
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
