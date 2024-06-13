// TODO?: refresh trait.

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

#[derive(Debug)]
pub struct Components {
	pub active_timer: Timer,
	pub setting: Setting,
	pub quote: Quoter,
	pub openai: OpenAi,
}
impl Components {
	pub fn init() -> Self {
		let active_timer = Timer::default();
		let setting = Setting::load().expect("setting must be loaded");
		let quote = Quoter::default();
		let openai = OpenAi::new(setting.ai.clone());

		Self { active_timer, setting, quote, openai }
	}
}
