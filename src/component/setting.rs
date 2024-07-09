// std
use std::{borrow::Cow, fs, path::PathBuf};
// crates.io
use app_dirs2::AppDataType;
use async_openai::config::OPENAI_API_BASE;
use eframe::egui::WidgetText;
use serde::{Deserialize, Serialize};
// self
use super::{function::Function, openai::Model};
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
	pub active_func: Function,
}
impl Default for General {
	fn default() -> Self {
		Self { font_size: 13., hide_on_lost_focus: true, active_func: Default::default() }
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
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Rewrite {
	pub additional_prompt: String,
}
impl Rewrite {
	pub fn prompt(&self) -> Cow<str> {
		const DEFAULT: &str = "As a language professor, assist me in refining text! \
			Amend any grammatical errors and enhance the language to sound more like a native speaker! \
			Text is always provided in format ```AiR\n$TEXT\n```! \
			$TEXT can be provided in any style, such as programming code! \
			Maintain the origin style but without the ```AiR\n\n``` surroundings! \
			Return the refined $TEXT only!";

		if self.additional_prompt.is_empty() {
			DEFAULT.into()
		} else {
			format!("{DEFAULT} {}", self.additional_prompt).into()
		}
	}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Translation {
	pub additional_prompt: String,
	pub a: Language,
	pub b: Language,
}
impl Translation {
	pub fn prompt(&self) -> Cow<str> {
		let default = format!(
			"As a language professor, assist me in translate text between {} and {}! \
			Amend any grammatical errors and enhance the language to sound more like a native speaker! \
			Text is always provided in format ```AiR\n$TEXT\n```! \
			$TEXT can be provided in any style, such as programming code! \
			Maintain the origin style but without the ```AiR\n\n``` surroundings! \
			Return the translated $TEXT only!",
			self.a.as_str(),
			self.b.as_str(),
		);

		if self.additional_prompt.is_empty() {
			default.into()
		} else {
			format!("{default} {}", self.additional_prompt).into()
		}
	}
}
impl Default for Translation {
	fn default() -> Self {
		Self { additional_prompt: Default::default(), a: Language::ZhCn, b: Language::EnGb }
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
	pub fn as_str(&self) -> &'static str {
		match self {
			Self::ZhCn => "zh-CN",
			Self::EnGb => "en-GB",
		}
	}

	pub fn all() -> [Self; 2] {
		[Self::ZhCn, Self::EnGb]
	}
}
#[allow(clippy::from_over_into)]
impl Into<WidgetText> for &Language {
	fn into(self) -> WidgetText {
		self.as_str().into()
	}
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
			rewrite: "ctrl+y".into(),
			rewrite_directly: "ctrl+u".into(),
			translate: "ctrl+i".into(),
			translate_directly: "ctrl+o".into(),
		}
	}
}
