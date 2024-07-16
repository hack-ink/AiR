// A fallback for users who use older settings.
mod fallback;

// std
use std::{borrow::Cow, fs, path::PathBuf};
// crates.io
use app_dirs2::AppDataType;
use async_openai::config::OPENAI_API_BASE;
use language::Language;
use serde::{Deserialize, Serialize};
use tracing::Level;
// self
use super::{function::Function, openai::Model};
use crate::{prelude::*, widget::ComboBoxItem, APP_INFO};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Setting {
	pub general: General,
	pub ai: Ai,
	pub chat: Chat,
	pub hotkeys: Hotkeys,
	pub development: Development,
}
impl Setting {
	pub fn path() -> Result<PathBuf> {
		Ok(app_dirs2::get_app_root(AppDataType::UserConfig, &APP_INFO)
			.map(|p| p.join("setting.toml"))?)
	}

	pub fn load() -> Result<Self> {
		let p = Self::path()?;

		tracing::info!("loading from {}", p.display());

		if !p.is_file() {
			tracing::warn!("it looks like you are running AiR for the first time, creating a new setting file from template");

			return Ok(Default::default());
		}

		let s = match fs::read_to_string(p) {
			Ok(s) => s,
			Err(e) => {
				tracing::error!("failed to load the setting due to: {e}");

				return Ok(Default::default());
			},
		};

		// TODO: https://github.com/hack-ink/AiR/issues/62.
		Ok(toml::from_str(&s)?)
	}

	pub fn save(&self) -> Result<()> {
		let p = Self::path()?;
		let d = p.parent().unwrap();

		if !d.is_dir() {
			fs::create_dir_all(d)?;
		}

		tracing::info!("saving to {}", p.display());

		Ok(fs::write(p, toml::to_string_pretty(self).unwrap())?)
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct General {
	pub font_size: f32,
	pub hide_on_lost_focus: bool,
	#[serde(default)]
	pub stick_to_top: bool,
	pub active_func: Function,
}
impl Default for General {
	fn default() -> Self {
		Self {
			font_size: 13.,
			hide_on_lost_focus: true,
			stick_to_top: Default::default(),
			active_func: Default::default(),
		}
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
		const DEFAULT: &str =
			"As a professional writer and language master, assist me in refining text! \
			Amend any grammatical errors and enhance the language to sound more like a native speaker! \
			Text is always provided in format `<AiR>$TEXT</AiR>`! \
			$TEXT can be provided in any style! \
			Discard the `<AiR></AiR>` tag! \
			But keep the indentation and line breaks format! \
			Extract the $TEXT and return the refined $TEXT only!";

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
	#[serde(deserialize_with = "fallback::translation_a")]
	pub a: Language,
	#[serde(deserialize_with = "fallback::translation_b")]
	pub b: Language,
}
impl Translation {
	// TODO: https://github.com/hack-ink/AiR/issues/41.
	pub fn prompt(&self) -> Cow<str> {
		let default = format!(
			"As a professional translator and language master, assist me in translating text! \
			I provide two languages, {} and {}! \
			Determine which language the text I give is in, and then translate accordingly. \
			Amend any grammatical errors and enhance the language to sound more like a native speaker! \
			Text is always provided in format `<AiR>$TEXT</AiR>`! \
			$TEXT can be provided in any style! \
			Discard the `<AiR></AiR>` tag! \
			But keep the indentation and line breaks format! \
			Extract the $TEXT and return the translated $TEXT only!",
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
			rewrite: "CTRL+T".into(),
			rewrite_directly: "CTRL+Y".into(),
			translate: "CTRL+U".into(),
			translate_directly: "CTRL+I".into(),
		}
	}
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Development {
	pub log_level: LogLevel,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LogLevel {
	Trace,
	Debug,
	Info,
	Warn,
	Error,
}
impl Default for LogLevel {
	fn default() -> Self {
		Self::Warn
	}
}
impl From<LogLevel> for Level {
	fn from(l: LogLevel) -> Self {
		match l {
			LogLevel::Trace => Level::TRACE,
			LogLevel::Debug => Level::DEBUG,
			LogLevel::Info => Level::INFO,
			LogLevel::Warn => Level::WARN,
			LogLevel::Error => Level::ERROR,
		}
	}
}
impl ComboBoxItem for LogLevel {
	type Array = [Self; Self::COUNT];

	const COUNT: usize = 5;

	fn all() -> Self::Array {
		[Self::Trace, Self::Debug, Self::Info, Self::Warn, Self::Error]
	}

	fn display(&self) -> Cow<str> {
		Cow::Borrowed(match self {
			Self::Trace => "Trace",
			Self::Debug => "Debug",
			Self::Info => "Info",
			Self::Warn => "Warn",
			Self::Error => "Error",
		})
	}
}
