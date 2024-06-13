// crates.io
use enigo::{Enigo, Settings};
// self
use crate::prelude::*;

#[derive(Debug)]
pub struct Keyboard {
	enigo: Enigo,
}
impl Keyboard {
	pub fn init() -> Result<Self> {
		Ok(Self { enigo: Enigo::new(&Settings::default()).map_err(EnigoError::NewCon)? })
	}

	// pub fn copy(&mut self) -> Result<()> {
	// 	self.enigo.key(Key::Other(0x37), Direction::Press).map_err(EnigoError::Input)?;
	// 	self.enigo.key(Key::Other(0x08), Direction::Click).map_err(EnigoError::Input)?;
	// 	self.enigo.key(Key::Other(0x37), Direction::Release).map_err(EnigoError::Input)?;
	//
	// 	Ok(())
	// }
}
