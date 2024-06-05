// std
use std::{fs, path::PathBuf};
// crates.io
use app_dirs2::{AppDataType, AppInfo};
use serde::{Deserialize, Serialize};
// self
use crate::{component::openai::Model, prelude::*};

const APP: AppInfo = AppInfo { name: "AiR", author: "xavier@inv.cafe" };

#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct Setting {
	pub(crate) general: General,
	pub(crate) ai: Ai,
}
impl Setting {
	pub(crate) fn path() -> Result<PathBuf> {
		Ok(app_dirs2::get_app_root(AppDataType::UserConfig, &APP).map(|p| p.join(".airrc"))?)
	}

	pub(crate) fn load() -> Result<Self> {
		let p = Self::path()?;

		tracing::info!("loading from {}", p.display());

		Ok(toml::from_str(&fs::read_to_string(p)?)?)
	}

	pub(crate) fn save(&self) -> Result<()> {
		let p = Self::path()?;

		tracing::info!("saving to {}", p.display());

		Ok(fs::write(p, toml::to_string_pretty(self).unwrap())?)
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct General {
	pub(crate) font_size: f32,
	pub(crate) hide_on_lost_focus: bool,
}
impl Default for General {
	fn default() -> Self {
		Self { font_size: 13., hide_on_lost_focus: true }
	}
}

// TODO: Support Google Gemini.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Ai {
	pub(crate) api_key: String,
	pub(crate) model: String,
	pub(crate) temperature: f32,
}
impl Ai {
	pub(crate) fn temperature_rounded(&self) -> f32 {
		(self.temperature * 10.).round() / 10.
	}
}
impl Default for Ai {
	fn default() -> Self {
		Self { api_key: Default::default(), model: Model::default().to_owned(), temperature: 0.7 }
	}
}
