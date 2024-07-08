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

// self
use crate::prelude::*;

#[derive(Debug)]
pub struct Components {
	pub setting: Setting,
	#[cfg(feature = "tokenizer")]
	pub tokenizer: Tokenizer,
}
impl Components {
	pub fn new() -> Result<Self> {
		let setting = Setting::load()?;

		// TODO: https://github.com/emilk/egui/discussions/4670.
		debug_assert_eq!(setting.ai.temperature, setting.ai.temperature * 10. / 10.);

		#[cfg(feature = "tokenizer")]
		let tokenizer = Tokenizer::new(setting.ai.model.as_str());

		Ok(Self {
			setting,
			#[cfg(feature = "tokenizer")]
			tokenizer,
		})
	}
}
