// crates.io
use eframe::egui::*;
// self
use crate::{
	component::{
		openai::Model,
		setting::{Ai as AiRaw, Setting as SettingRaw, *},
	},
	widget::ApiKey,
};

#[derive(Debug)]
pub struct Setting {
	pub general: General,
	pub ai: Ai,
}
impl Setting {
	fn set_font_sizes(&self, ctx: &Context) {
		ctx.style_mut(|s| {
			s.text_styles.values_mut().for_each(|s| s.size = self.general.font_size);
		});
	}

	pub fn draw(&mut self, ui: &mut Ui) {
		ui.collapsing("General", |ui| {
			Grid::new("General").show(ui, |ui| {
				// Font size.
				// TODO: Adjust widget's length.
				ui.label("Font Size");
				if ui
					.add(
						Slider::new(&mut self.general.font_size, 9_f32..=16.)
							.step_by(1.)
							.fixed_decimals(0),
					)
					.changed()
				{
					self.set_font_sizes(ui.ctx());
				}
				ui.end_row();
			});
		});
		ui.collapsing("AI", |ui| {
			Grid::new("AI").num_columns(2).striped(true).show(ui, |ui| {
				// API key.
				ui.label("API Key");
				let width = ui
					.horizontal(|ui| {
						let size = ui.available_size();
						let w_text = ui.add_sized(
							(size.x - 56., size.y),
							TextEdit::singleline(&mut self.ai.raw.api_key)
								.password(self.ai.widget.visibility),
						);

						// TODO?: Persistent OpenAI client.
						// if w_text.changed() {}
						if ui.button(&self.ai.widget.label).clicked() {
							self.ai.widget.clicked();
						}

						w_text.rect.width()
					})
					.inner;
				ui.spacing_mut().slider_width = width;
				ui.end_row();

				// Model.
				ui.label("Model");
				ComboBox::from_id_source("Model").selected_text(&self.ai.raw.model).show_ui(
					ui,
					|ui| {
						Model::all().iter().for_each(|m| {
							ui.selectable_value(&mut self.ai.raw.model, (*m).to_owned(), *m);
						});
					},
				);
				ui.end_row();

				// Temperature.
				ui.label("Temperature");
				ui.add(
					Slider::new(&mut self.ai.raw.temperature, 0_f32..=2.)
						.fixed_decimals(1)
						.step_by(0.1),
				);
				ui.end_row();
			});
		});
		ui.collapsing("Hotkey", |_ui| {});
	}

	pub fn save(&mut self) {
		if let Err(e) = SettingRaw::from(&*self).save() {
			tracing::error!("{e:?}");
		}
	}
}
impl Default for Setting {
	fn default() -> Self {
		SettingRaw::load().unwrap_or_default().into()
	}
}
impl From<SettingRaw> for Setting {
	fn from(v: SettingRaw) -> Self {
		Self { general: v.general, ai: v.ai.into() }
	}
}
impl From<&Setting> for SettingRaw {
	fn from(v: &Setting) -> Self {
		Self {
			general: General {
				font_size: v.general.font_size,
				hide_on_lost_focus: v.general.hide_on_lost_focus,
			},
			ai: (&v.ai).into(),
		}
	}
}

#[derive(Debug)]
pub struct Ai {
	pub raw: AiRaw,
	pub widget: ApiKey,
}
impl From<AiRaw> for Ai {
	fn from(v: AiRaw) -> Self {
		Self { raw: v, widget: ApiKey::default() }
	}
}
impl From<&Ai> for AiRaw {
	fn from(v: &Ai) -> Self {
		v.raw.clone()
	}
}
