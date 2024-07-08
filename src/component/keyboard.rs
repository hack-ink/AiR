// crates.io
use enigo::{Direction, Enigo, Key, Keyboard as _, Settings};
#[cfg(all(unix, not(target_os = "macos")))] use xkeysym::key::c;
// self
use crate::prelude::*;

#[derive(Debug)]
pub struct Keyboard(pub Enigo);
impl Keyboard {
	pub fn init() -> Result<Self> {
		Ok(Self(Enigo::new(&Settings::default()).map_err(EnigoError::NewCon)?))
	}

	pub fn copy(&mut self) -> Result<()> {
		self.0.key(Key::Meta, Direction::Press).map_err(EnigoError::Input)?;
		// TODO: create a `CGKeyCode` table for macOS in `build.rs`.
		#[cfg(target_os = "macos")]
		self.0.key(Key::Other(0x08), Direction::Click).map_err(EnigoError::Input)?;
		// TODO: Windows.
		#[cfg(all(unix, not(target_os = "macos")))]
		self.0.key(Key::Other(c), Direction::Click).map_err(EnigoError::Input)?;
		self.0.key(Key::Meta, Direction::Release).map_err(EnigoError::Input)?;

		Ok(())
	}

	pub fn text(&mut self, text: &str) -> Result<()> {
		Ok(self.0.text(text).map_err(EnigoError::Input)?)
	}
}
