mod main;
use main::*;

mod setting;
use setting::*;

// crates.io
use eframe::egui::*;

#[derive(Debug)]
pub struct Data {
	panels: Panels,
	main: Main,
	setting: Setting,
}
impl Data {
	fn set_font(ctx: &Context) {
		let mut fonts = FontDefinitions::default();

		fonts.font_data.insert(
			"Monaspace Radon Var".into(),
			FontData::from_static(include_bytes!("../asset/CascadiaCode.ttf")),
		);
		fonts
			.families
			.entry(FontFamily::Proportional)
			.or_default()
			.insert(0, "Monaspace Radon Var".into());
		fonts
			.families
			.entry(FontFamily::Monospace)
			.or_default()
			.insert(0, "Monaspace Radon Var".into());

		ctx.set_fonts(fonts);
	}

	pub fn new(ctx: &Context) -> Self {
		Self::set_font(ctx);

		let setting = Setting::default();
		let main = Main::from_setting(&setting);

		Self { panels: Default::default(), main, setting }
	}

	pub fn draw(&mut self, ui: &mut Ui) {
		ui.horizontal(|ui| {
			ui.selectable_value(&mut self.panels, Panels::Main, "Main");
			ui.separator();
			ui.selectable_value(&mut self.panels, Panels::Setting, "Setting");
			ui.separator();
		});
		ui.separator();

		match self.panels {
			Panels::Main => self.main.draw(ui),
			Panels::Setting => self.setting.draw(ui),
		}
	}

	pub fn refresh(&mut self) {
		self.main.refresh();
	}

	pub fn save(&mut self) {
		self.setting.save();
	}
}

#[derive(Debug, PartialEq, Eq)]
enum Panels {
	Main,
	Setting,
}
impl Default for Panels {
	fn default() -> Self {
		Self::Main
		// Self::Setting
	}
}
