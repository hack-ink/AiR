// TODO?: refresh trait.

pub mod tokenizer;
use tokenizer::Tokenizer;

pub mod function;

pub mod keyboard;

pub mod net;

pub mod openai;
use openai::OpenAi;

mod quote;
use quote::Quoter;

pub mod setting;
use setting::Setting;

pub mod timer;
use timer::Timer;

pub mod util;

// std
use std::sync::Arc;
// self
use crate::prelude::*;

#[derive(Debug)]
pub struct Components {
	pub active_timer: Timer,
	pub setting: Setting,
	pub quote: Quoter,
	pub tokenizer: Tokenizer,
	pub openai: Arc<OpenAi>,
}
impl Components {
	pub fn init() -> Result<Self> {
		let active_timer = Timer::default();
		let setting = Setting::load()?;
		let quote = Quoter::default();
		let tokenizer = Tokenizer::new(setting.ai.model.as_str());
		// TODO: no clone.
		let openai = Arc::new(OpenAi::new(setting.ai.clone()));

		Ok(Self { active_timer, setting, quote, tokenizer, openai })
	}
}
