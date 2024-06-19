#[cfg(feature = "tokenizer")] pub mod tokenizer;
#[cfg(feature = "tokenizer")] use tokenizer::Tokenizer;

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
// crates.io
use tokio::sync::Mutex;
// self
use crate::prelude::*;

#[derive(Debug)]
pub struct Components {
	pub setting: Setting,
	#[cfg(feature = "tokenizer")]
	pub tokenizer: Tokenizer,
	pub openai: Arc<Mutex<OpenAi>>,
}
impl Components {
	pub fn init() -> Result<Self> {
		let setting = Setting::load()?;
		#[cfg(feature = "tokenizer")]
		let tokenizer = Tokenizer::new(setting.ai.model.as_str());
		let openai = Arc::new(Mutex::new(OpenAi::new(setting.ai.clone())));

		Ok(Self {
			setting,
			#[cfg(feature = "tokenizer")]
			tokenizer,
			openai,
		})
	}

	// TODO?: move to somewhere else.
	pub fn reload_openai(&self) {
		tracing::info!("reloading openai component");

		self.openai.blocking_lock().reload(self.setting.ai.clone());
	}
}
