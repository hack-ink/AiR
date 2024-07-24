// A fallback for users who use older settings.
mod fallback;

// std
use std::{borrow::Cow, fs, path::PathBuf};
// crates.io
use app_dirs2::AppDataType;
use async_openai::config::OPENAI_API_BASE;
use global_hotkey::hotkey::HotKey;
use language::Language;
use serde::{Deserialize, Serialize};
use tracing::Level;
// self
use super::{function::Function, openai::Model};
use crate::{component::keyboard::Keys, prelude::*, widget::ComboBoxItem, APP_INFO};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
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

		Ok(fs::write(p, toml::to_string_pretty(self).expect("write to file must succeed"))?)
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct General {
	pub font_size: f32,
	pub hide_on_lost_focus: bool,
	pub stick_to_top: bool,
	pub notification_sound: bool,
}
impl Default for General {
	fn default() -> Self {
		Self {
			font_size: 13.,
			hide_on_lost_focus: true,
			stick_to_top: Default::default(),
			notification_sound: true,
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
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
#[serde(default, rename_all = "kebab-case")]
pub struct Chat {
	pub activated_function: Function,
	pub rewrite: Rewrite,
	pub translation: Translation,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct Rewrite {
	pub additional_prompt: String,
}
impl Rewrite {
	pub fn prompt(&self) -> Cow<str> {
		const DEFAULT: &str = "Rewrite the content enclosed within the <AiR></AiR> tags.";

		if self.additional_prompt.is_empty() {
			DEFAULT.into()
		} else {
			format!(
				"{DEFAULT} \
				{} \
				The text could be in any format (including code comments). \
				Return only the rewritten version without any additional information, commentary or the tag.",
				self.additional_prompt
			)
			.into()
		}
	}
}
impl Default for Rewrite {
	fn default() -> Self {
		Self { additional_prompt: "Keep the original text format as much as possible.".into() }
	}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
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
			"Translate the content enclosed within the <AiR></AiR> tags between Language {} and Language {}, \
			based on the language it is currently written in.",
			self.a.as_str(),
			self.b.as_str(),
		);

		if self.additional_prompt.is_empty() {
			default.into()
		} else {
			format!(
				"{default} \
				{} \
				The text could be in any format (including code comments). \
				Return only the translated version without any additional information, commentary or the tag.",
				self.additional_prompt
			)
			.into()
		}
	}
}
impl Default for Translation {
	fn default() -> Self {
		Self {
			additional_prompt: "Make it sound more native.".into(),
			a: Language::ZhCn,
			b: Language::EnGb,
		}
	}
}

// We do not derive `serde(default)` for `Hotkeys`.
//
// If a user intends to leave a hotkey empty, then the hotkey should be set to `None`.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Hotkeys {
	pub rewrite: MaybeHotkey,
	pub rewrite_directly: MaybeHotkey,
	pub translate: MaybeHotkey,
	pub translate_directly: MaybeHotkey,
}
impl Default for Hotkeys {
	fn default() -> Self {
		#[cfg(target_os = "macos")]
		let hks = Self {
			rewrite: MaybeHotkey::from_str_raw("CTRL+T"),
			rewrite_directly: MaybeHotkey::from_str_raw("CTRL+Y"),
			translate: MaybeHotkey::from_str_raw("CTRL+U"),
			translate_directly: MaybeHotkey::from_str_raw("CTRL+I"),
		};
		#[cfg(not(target_os = "macos"))]
		let hks = Self {
			rewrite: MaybeHotkey::from_str_raw("ALT+T"),
			rewrite_directly: MaybeHotkey::from_str_raw("ALT+Y"),
			translate: MaybeHotkey::from_str_raw("ALT+U"),
			translate_directly: MaybeHotkey::from_str_raw("ALT+I"),
		};

		hks
	}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MaybeHotkey(#[serde(skip_serializing_if = "Option::is_none")] pub Option<String>);
impl MaybeHotkey {
	pub fn from_str_raw(s: &str) -> Self {
		Self(Some(s.to_owned()))
	}

	pub fn from_string_infallible(s: String) -> Self {
		let hk = Self::from_str_raw(&s);

		match hk.validate() {
			Ok(_) => hk,
			Err(e) => {
				tracing::warn!("failed to validate hotkey due to: {e}");

				Self(None)
			},
		}
	}

	pub fn as_str(&self) -> &str {
		self.0.as_deref().unwrap_or("None")
	}

	pub fn validate(&self) -> Result<Option<(HotKey, Keys)>> {
		if let Some(hk_raw) = &self.0 {
			let hk = hk_raw.parse().map_err(GlobalHotKeyError::Parse)?;
			let ks = hk_raw.parse()?;

			Ok(Some((hk, ks)))
		} else {
			Ok(None)
		}
	}
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
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

	fn selectable_str(&self) -> Cow<str> {
		match self {
			Self::Trace => "Trace",
			Self::Debug => "Debug",
			Self::Info => "Info",
			Self::Warn => "Warn",
			Self::Error => "Error",
		}
		.into()
	}
}
