// TODO?: refresh trait.

pub mod tokenizer;
use tokenizer::Tokenizer;

pub mod function;

pub mod keyboard;

pub mod net;

pub mod openai;
use openai::OpenAi;

pub mod quote;

pub mod setting;
use setting::Setting;

pub mod util;

// std
use std::sync::Arc;
// self
use crate::prelude::*;

#[derive(Debug)]
pub struct Components {
	pub setting: Setting,
	pub tokenizer: Tokenizer,
	pub openai: Arc<OpenAi>,
}
impl Components {
	pub fn init() -> Result<Self> {
		let setting = Setting::load()?;
		let tokenizer = Tokenizer::new(setting.ai.model.as_str());
		// TODO: no clone.
		let openai = Arc::new(OpenAi::new(setting.ai.clone()));

		Ok(Self { setting, tokenizer, openai })
	}
}
