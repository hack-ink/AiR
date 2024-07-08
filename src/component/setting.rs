// std
use std::{borrow::Cow, fs, path::PathBuf};
// crates.io
use app_dirs2::AppDataType;
use async_openai::config::OPENAI_API_BASE;
use serde::{Deserialize, Serialize};
// self
use super::openai::Model;
use crate::{prelude::*, APP_INFO};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Setting {
	pub general: General,
	pub ai: Ai,
	pub chat: Chat,
	pub hotkeys: Hotkeys,
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

// TODO?: implement a `Prompt` trait.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Chat {
	pub rewrite: Rewrite,
	pub translation: Translation,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Rewrite {
	pub prompt: String,
}
impl Rewrite {
	pub fn prompt(&self) -> Cow<str> {
		Cow::Borrowed(&self.prompt)
	}
}
impl Default for Rewrite {
	fn default() -> Self {
		Self {
			prompt: "As language professor, assist me in refining this text. \
				Amend any grammatical errors, \
				enhance the language to sound more like a native speaker and keep the origin format. \
				Just provide the refined text only, without any other things:"
				.into(),
		}
	}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Translation {
	pub prompt: String,
	pub a: Language,
	pub b: Language,
}
impl Translation {
	pub fn prompt(&self) -> Cow<str> {
		Cow::Owned(format!(
			"Assist me in translate this text between {} and {}. {}",
			self.a.as_str(),
			self.b.as_str(),
			self.prompt
		))
	}
}
impl Default for Translation {
	fn default() -> Self {
		Self {
			prompt: "As a language professor, amend any grammatical errors, \
				enhance the language to sound more like a native speaker and keep the origin format. \
				Provide the translated text only, without any other things:"
				.into(),
			a: Language::ZhCn,
			b: Language::EnGb,
		}
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

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Hotkeys {
	pub rewrite: String,
	pub rewrite_directly: String,
	pub translate: String,
	pub translate_directly: String,
}
impl Default for Hotkeys {
	fn default() -> Self {
		Self {
			rewrite: "Ctrl+Y".into(),
			rewrite_directly: "Ctrl+U".into(),
			translate: "Ctrl+I".into(),
			translate_directly: "Ctrl+O".into(),
		}
	}
}
