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
	// Keyboard didn't implement `Send`, can't use it between threads.
	// pub keyboard: Arc<Mutex<Keyboard>>,
	// TODO?: move the lock to somewhere else.
	pub openai: Arc<Mutex<OpenAi>>,
	#[cfg(feature = "tokenizer")]
	pub tokenizer: Tokenizer,
}
impl Components {
	pub fn init() -> Result<Self> {
		let setting = Setting::load()?;

		// TODO: https://github.com/emilk/egui/discussions/4670.
		debug_assert_eq!(setting.ai.temperature, setting.ai.temperature * 10. / 10.);

		let openai = Arc::new(Mutex::new(OpenAi::new(setting.ai.clone())));
		#[cfg(feature = "tokenizer")]
		let tokenizer = Tokenizer::new(setting.ai.model.as_str());

		Ok(Self {
			setting,
			openai,
			#[cfg(feature = "tokenizer")]
			tokenizer,
		})
	}

	// TODO?: move to somewhere else.
	pub fn reload_openai(&self) {
		tracing::info!("reloading openai component");

		self.openai.blocking_lock().reload(self.setting.ai.clone());
	}
}
