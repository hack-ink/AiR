// crates.io
use enigo::{Enigo, Keyboard as _, Settings};
// self
use crate::prelude::*;

#[derive(Debug)]
pub struct Keyboard {
	pub enigo: Enigo,
}
impl Keyboard {
	pub fn init() -> Result<Self> {
		let enigo = Enigo::new(&Settings::default()).map_err(EnigoError::NewCon)?;

		Ok(Self { enigo })
	}

	// pub fn copy(&mut self) -> Result<()> {
	// 	self.enigo.key(Key::Other(0x37), Direction::Press).map_err(EnigoError::Input)?;
	// 	self.enigo.key(Key::Other(0x08), Direction::Click).map_err(EnigoError::Input)?;
	// 	self.enigo.key(Key::Other(0x37), Direction::Release).map_err(EnigoError::Input)?;
	//
	// 	Ok(())
	// }

	pub fn text(&mut self, text: &str) -> Result<()> {
		Ok(self.enigo.text(text).map_err(EnigoError::Input)?)
	}
}
