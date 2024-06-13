// crates.io
use eframe::egui::*;
// self
use super::super::UiT;
use crate::{air::AiRContext, component::openai::Model};

#[derive(Debug, Default)]
pub struct Setting {
	pub api_key_widget: ApiKeyWidget,
}
impl Setting {
	fn set_font_sizes(&self, ctx: &AiRContext) {
		ctx.egui_ctx.style_mut(|s| {
			s.text_styles
				.values_mut()
				.for_each(|s| s.size = ctx.components.setting.general.font_size);
		});
	}
}
impl UiT for Setting {
	fn draw(&mut self, ui: &mut Ui, ctx: &mut AiRContext) {
		ui.collapsing("General", |ui| {
			Grid::new("General").show(ui, |ui| {
				// Font size.
				// TODO: adjust api_key_widget's length.
				ui.label("Font Size");
				if ui
					.add(
						Slider::new(&mut ctx.components.setting.general.font_size, 9_f32..=16.)
							.step_by(1.)
							.fixed_decimals(0),
					)
					.changed()
				{
					self.set_font_sizes(ctx);
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
							TextEdit::singleline(&mut ctx.components.setting.ai.api_key)
								.password(self.api_key_widget.visibility),
						);

						// TODO?: persistent OpenAI client.
						// if w_text.changed() {}
						if ui.button(&self.api_key_widget.label).clicked() {
							self.api_key_widget.clicked();
						}

						w_text.rect.width()
					})
					.inner;
				ui.spacing_mut().slider_width = width;
				ui.end_row();

				// Model.
				ui.label("Model");
				ComboBox::from_id_source("Model")
					.selected_text(&ctx.components.setting.ai.model)
					.show_ui(ui, |ui| {
						Model::all().iter().for_each(|m| {
							ui.selectable_value(
								&mut ctx.components.setting.ai.model,
								m.to_owned(),
								m.as_str(),
							);
						});
					});
				ui.end_row();

				// Temperature.
				ui.label("Temperature");
				ui.add(
					Slider::new(&mut ctx.components.setting.ai.temperature, 0_f32..=2.)
						.fixed_decimals(1)
						.step_by(0.1),
				);
				ui.end_row();
			});
		});
		// ui.collapsing("Hotkey", |_ui| {});
	}
}

#[derive(Debug)]
pub struct ApiKeyWidget {
	pub label: String,
	pub visibility: bool,
}
impl ApiKeyWidget {
	pub fn clicked(&mut self) {
		self.label = match self.label.as_str() {
			"show" => "hide".into(),
			"hide" => "show".into(),
			_ => unreachable!(),
		};
		self.visibility = !self.visibility;
	}
}
impl Default for ApiKeyWidget {
	fn default() -> Self {
		Self { label: "show".into(), visibility: true }
	}
}
