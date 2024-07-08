#[cfg(feature = "tokenizer")] pub mod tokenizer;
#[cfg(feature = "tokenizer")] use tokenizer::Tokenizer;

pub mod function;

pub mod keyboard;

pub mod net;

pub mod openai;

pub mod quote;

pub mod setting;
use setting::Setting;

pub mod util;

// std
use std::fmt::{Debug, Formatter, Result as FmtResult};
// crates.io
use arboard::Clipboard;
// self
use crate::prelude::*;

pub struct Components {
	pub clipboard: Clipboard,
	pub setting: Setting,
	#[cfg(feature = "tokenizer")]
	pub tokenizer: Tokenizer,
}
impl Components {
	pub fn new() -> Result<Self> {
		let clipboard = Clipboard::new()?;
		let setting = Setting::load()?;

		// TODO: https://github.com/emilk/egui/discussions/4670.
		debug_assert_eq!(setting.ai.temperature, setting.ai.temperature * 10. / 10.);

		#[cfg(feature = "tokenizer")]
		let tokenizer = Tokenizer::new(setting.ai.model.as_str());

		Ok(Self {
			clipboard,
			setting,
			#[cfg(feature = "tokenizer")]
			tokenizer,
		})
	}
}
impl Debug for Components {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		let mut s = f.debug_struct("Components");

		s.field("clipboard", &"..").field("setting", &self.setting);
		#[cfg(feature = "tokenizer")]
		s.field("tokenizer", &self.tokenizer);

		s.finish()
	}
}
