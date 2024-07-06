// std
use std::{fs, path::PathBuf};
// crates.io
use app_dirs2::AppDataType;
use async_openai::config::OPENAI_API_BASE;
use eframe::egui::WidgetText;
use serde::{Deserialize, Serialize};
// self
use super::openai::Model;
use crate::{prelude::*, APP_INFO};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Setting {
	pub general: General,
	pub ai: Ai,
	pub translation: Translation,
}
impl Setting {
	pub fn path() -> Result<PathBuf> {
		Ok(app_dirs2::get_app_root(AppDataType::UserConfig, &APP_INFO)
			.map(|p| p.join("setting.toml"))?)
	}

	pub fn load() -> Result<Self> {
		let p = Self::path()?;

		tracing::info!("loading from {}", p.display());

		let s = match fs::read_to_string(p) {
			Ok(s) => s,
			Err(e) => {
				tracing::error!("failed to load the setting due to: {e}");

				return Ok(Default::default());
			},
		};

		Ok(toml::from_str(&s)?)
	}

	pub fn save(&self) -> Result<()> {
		let p = Self::path()?;

		tracing::info!("saving to {}", p.display());

		Ok(fs::write(p, toml::to_string_pretty(self).unwrap())?)
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct General {
	pub font_size: f32,
	pub hide_on_lost_focus: bool,
}
impl Default for General {
	fn default() -> Self {
		Self { font_size: 13., hide_on_lost_focus: true }
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Ai {
	pub api_base: String,
	pub api_key: String,
	pub model: Model,
	pub temperature: f32,
}
impl Default for Ai {
	fn default() -> Self {
		Self {
			api_base: OPENAI_API_BASE.into(),
			api_key: Default::default(),
			model: Model::default(),
			temperature: 0.7,
		}
	}
}

// TODO: add a super type for all the settings.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Translation {
	pub source: Language,
	pub target: Language,
}
impl Default for Translation {
	fn default() -> Self {
		Self { source: Language::ZhCn, target: Language::EnGb }
	}
}
// https://www.alchemysoftware.com/livedocs/ezscript/Topics/Catalyst/Language.htm
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Language {
	// Chinese (Simplified, People's Republic of China).
	ZhCn,
	// English (United Kingdom).
	EnGb,
}
impl Language {
	pub fn all() -> [Self; 2] {
		[Self::ZhCn, Self::EnGb]
	}

	pub fn as_str(&self) -> &'static str {
		match self {
			Self::ZhCn => "zh-CN",
			Self::EnGb => "en-GB",
		}
	}
}
#[allow(clippy::from_over_into)]
impl Into<WidgetText> for &Language {
	fn into(self) -> WidgetText {
		self.as_str().into()
	}
}
