mod main;
use main::*;

mod preference;
use preference::*;

// crates.io
use eframe::egui::*;

#[derive(Debug, Default)]
pub(super) struct Uii {
	panels: Panels,
	main: Main,
	preference: Preference,
}
impl Uii {
	fn set_font(ctx: &Context) {
		let mut fonts = FontDefinitions::default();

		fonts.font_data.insert(
			"Monaspace Radon Var".into(),
			FontData::from_static(include_bytes!(
				"../asset/MonaspaceRadonVarVF[wght,wdth,slnt].ttf"
			)),
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

	pub(super) fn new(ctx: &Context) -> Self {
		Self::set_font(ctx);

		Default::default()
	}

	pub(super) fn draw(&mut self, ui: &mut Ui) {
		ui.horizontal(|ui| {
			ui.selectable_value(&mut self.panels, Panels::Main, "Main");
			ui.separator();
			ui.selectable_value(&mut self.panels, Panels::Preference, "Preference");
			ui.separator();
		});
		ui.separator();

		match self.panels {
			Panels::Main => self.main.draw(ui),
			Panels::Preference => self.preference.draw(ui),
		}
	}

	pub(super) fn try_update(&mut self) {
		self.main.try_update();
	}
}

#[derive(Debug, PartialEq, Eq)]
enum Panels {
	Main,
	Preference,
}
impl Default for Panels {
	fn default() -> Self {
		Self::Main
		// Self::Preference
	}
}
