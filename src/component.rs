pub mod audio;

pub mod function;

pub mod keyboard;

pub mod net;

pub mod openai;

pub mod os;
use os::Os;

pub mod quote;

pub mod setting;
use setting::Setting;

// std
use std::fmt::{Debug, Formatter, Result as FmtResult};
// crates.io
use arboard::Clipboard;
// self
use crate::prelude::*;

pub struct Components {
	pub os: Os,
	pub clipboard: Clipboard,
	pub setting: Setting,
}
impl Components {
	pub fn new() -> Result<Self> {
		let os = Os::new();
		let clipboard = Clipboard::new()?;
		let setting = Setting::load()?;

		// TODO: https://github.com/emilk/egui/discussions/4670.
		debug_assert_eq!(setting.ai.temperature, setting.ai.temperature * 10. / 10.);

		Ok(Self { os, clipboard, setting })
	}
}
impl Debug for Components {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		f.debug_struct("Components")
			.field("os", &self.os)
			.field("clipboard", &"..")
			.field("setting", &self.setting)
			.finish()
	}
}
