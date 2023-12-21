// crates.io
use eframe::egui::*;
// air
use crate::widget::ApiKey;

#[derive(Debug)]
pub(super) struct Preference {
	api_key: ApiKey,
	temperature: f32,
	font_size: f32,
}
impl Preference {
	fn set_font_sizes(&self, ctx: &Context) {
		ctx.style_mut(|s| {
			s.text_styles.values_mut().for_each(|s| s.size = self.font_size);
		});
	}

	pub(super) fn draw(&mut self, ui: &mut Ui) {
		ui.collapsing("General", |ui| {
			Grid::new("some_unique_id").show(ui, |ui| {
				{
					ui.label("API Key");

					let width = ui
						.horizontal(|ui| {
							let w_text = ui.add(
								TextEdit::singleline(&mut self.api_key.value)
									.password(self.api_key.password),
							);

							if ui.button(&self.api_key.label).clicked() {
								self.api_key.clicked();
							}

							w_text.rect.width()
						})
						.inner;

					ui.spacing_mut().slider_width = width;
				}
				ui.end_row();

				{
					ui.label("Temperature");
					ui.add(
						Slider::new(&mut self.temperature, 0_f32..=1.)
							.step_by(0.01)
							.max_decimals(2),
					);
				}
				ui.end_row();

				{
					ui.label("Font Size");
					if ui
						.add(
							Slider::new(&mut self.font_size, 9_f32..=16.)
								.step_by(0.1)
								.max_decimals(1),
						)
						.changed()
					{
						self.set_font_sizes(ui.ctx());
					}
				}
				ui.end_row();
			});
		});
		ui.collapsing("Hotkey", |_ui| {});
	}
}
impl Default for Preference {
	fn default() -> Self {
		Self { api_key: Default::default(), temperature: 0.7, font_size: 13. }
	}
}
