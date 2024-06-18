// crates.io
use eframe::egui::*;
// self
use super::super::UiT;
use crate::{
	air::AiRContext,
	component::{openai::Model, setting::Language},
};

// TODO: when to reload the API client.
#[derive(Debug, Default)]
pub struct Setting {
	pub api_key: ApiKeyWidget,
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
			Grid::new("General").num_columns(2).striped(true).show(ui, |ui| {
				ui.label("Font Size");
				ui.horizontal(|ui| {
					ui.spacing_mut().slider_width = ui.available_width() - 56.;

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
				});
				ui.end_row();
			});
		});

		ui.collapsing("AI", |ui| {
			Grid::new("AI").num_columns(2).striped(true).show(ui, |ui| {
				ui.label("API Base");
				let size = ui
					.horizontal(|ui| {
						let mut size = ui.available_size();

						size.x -= 56.;

						ui.add_sized(
							size,
							TextEdit::singleline(&mut ctx.components.setting.ai.api_base),
						);

						size
					})
					.inner;
				ui.end_row();

				ui.label("API Key");
				ui.horizontal(|ui| {
					ui.add_sized(
						size,
						TextEdit::singleline(&mut ctx.components.setting.ai.api_key)
							.password(self.api_key.visibility),
					);

					if ui.button(&self.api_key.label).clicked() {
						self.api_key.clicked();
					}
				});
				ui.end_row();

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

				ui.label("Temperature");
				ui.spacing_mut().slider_width = size.x;
				ui.add(
					Slider::new(&mut ctx.components.setting.ai.temperature, 0_f32..=2.)
						.fixed_decimals(1)
						.step_by(0.1),
				);
				ui.end_row();
			});
		});

		ui.collapsing("Translation", |ui| {
			Grid::new("Translation").num_columns(2).striped(true).show(ui, |ui| {
				ui.label("Source");
				ComboBox::from_id_source("Source")
					.selected_text(&ctx.components.setting.translation.source)
					.show_ui(ui, |ui| {
						Language::all().iter().for_each(|l| {
							ui.selectable_value(
								&mut ctx.components.setting.translation.source,
								l.to_owned(),
								l.as_str(),
							);
						});
					});
				ui.end_row();

				ui.label("Target");
				ComboBox::from_id_source("Target")
					.selected_text(&ctx.components.setting.translation.target)
					.show_ui(ui, |ui| {
						Language::all().iter().for_each(|l| {
							ui.selectable_value(
								&mut ctx.components.setting.translation.target,
								l.to_owned(),
								l.as_str(),
							);
						});
					});
				ui.end_row();
			});
		});
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
