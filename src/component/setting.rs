// std
use std::{fs, path::PathBuf};
// crates.io
use app_dirs2::{AppDataType, AppInfo};
use serde::{Deserialize, Serialize};
// self
use crate::{component::openai::Model, prelude::*};

const APP: AppInfo = AppInfo { name: "AiR", author: "xavier@inv.cafe" };

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Setting {
	pub general: General,
	pub ai: Ai,
}
impl Setting {
	pub fn path() -> Result<PathBuf> {
		Ok(app_dirs2::get_app_root(AppDataType::UserConfig, &APP).map(|p| p.join(".airrc"))?)
	}

	pub fn load() -> Result<Self> {
		let p = Self::path()?;

		tracing::info!("loading from {}", p.display());

		Ok(toml::from_str(&fs::read_to_string(p)?)?)
	}

	pub fn save(&self) -> Result<()> {
		let p = Self::path()?;

		tracing::info!("saving to {}", p.display());

		Ok(fs::write(p, toml::to_string_pretty(self).unwrap())?)
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct General {
	pub font_size: f32,
	pub hide_on_lost_focus: bool,
}
impl Default for General {
	fn default() -> Self {
		Self { font_size: 13., hide_on_lost_focus: true }
	}
}

// TODO: Support Google Gemini.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ai {
	// TODO: custom API endpoint.
	pub api_key: String,
	pub model: Model,
	pub temperature: f32,
}
impl Ai {
	pub fn temperature_rounded(&self) -> f32 {
		(self.temperature * 10.).round() / 10.
	}
}
impl Default for Ai {
	fn default() -> Self {
		Self { api_key: Default::default(), model: Model::default(), temperature: 0.7 }
	}
}
