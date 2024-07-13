// std
use std::str::FromStr;
// crates.io
use enigo::{Direction, Enigo, Key, Keyboard as _, Settings};
// self
use crate::prelude::*;

#[derive(Debug)]
pub struct Keyboard(pub Enigo);
impl Keyboard {
	pub fn new() -> Result<Self> {
		Ok(Self(Enigo::new(&Settings::default()).map_err(EnigoError::NewCon)?))
	}

	pub fn copy(&mut self) -> Result<()> {
		let modifier = if cfg!(target_os = "macos") { Key::Meta } else { Key::Control };

		self.0.key(modifier, Direction::Press).map_err(EnigoError::Input)?;
		self.0.key(key_of('C')?, Direction::Click).map_err(EnigoError::Input)?;
		self.0.key(modifier, Direction::Release).map_err(EnigoError::Input)?;

		Ok(())
	}

	pub fn release_keys(&mut self, keys: Keys) -> Result<()> {
		for k in keys.0 {
			self.0.key(k, Direction::Release).map_err(EnigoError::Input)?;
		}

		Ok(())
	}

	pub fn text(&mut self, text: &str) -> Result<()> {
		Ok(self.0.text(text).map_err(EnigoError::Input)?)
	}
}

#[derive(Clone, Debug)]
pub struct Keys(pub Vec<Key>);
impl FromStr for Keys {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut keys = Vec::new();

		for k in s.to_uppercase().split('+') {
			let k = match k {
				"CTRL" | "CONTROL" => Key::Control,
				"SHIFT" => Key::Shift,
				"ALT" => Key::Alt,
				"COMMAND" | "META" | "SUPER" => Key::Meta,
				k if k.len() == 1 => key_of(k.chars().next().expect("`k` must be `char`"))?,
				k => return Err(Error::UnsupportedKey(k.to_owned())),
			};

			keys.push(k);
		}

		Ok(Self(keys))
	}
}

// We can't use [`enigo::Key::Unicode`], it will cause panic.
// Don't know why, maybe that can only be used in main thread.
fn key_of(key: char) -> Result<Key> {
	// TODO: create a `CGKeyCode` table for macOS in `build.rs`.
	// Currently, we only support limited keys on macOS from:
	// https://eastmanreference.com/complete-list-of-applescript-key-codes.
	#[cfg(target_os = "macos")]
	let k = Key::Other(match key {
		'A' => 0,
		'S' => 1,
		'D' => 2,
		'F' => 3,
		'H' => 4,
		'G' => 5,
		'Z' => 6,
		'X' => 7,
		'C' => 8,
		'V' => 9,
		'B' => 11,
		'Q' => 12,
		'W' => 13,
		'E' => 14,
		'R' => 15,
		'Y' => 16,
		'T' => 17,
		'1' => 18,
		'2' => 19,
		'3' => 20,
		'4' => 21,
		'6' => 22,
		'5' => 23,
		'=' => 24,
		'9' => 25,
		'7' => 26,
		'-' => 27,
		'8' => 28,
		'0' => 29,
		']' => 30,
		'O' => 31,
		'U' => 32,
		'[' => 33,
		'I' => 34,
		'P' => 35,
		'L' => 37,
		'J' => 38,
		'\'' => 39,
		'K' => 40,
		';' => 41,
		'\\' => 42,
		',' => 43,
		'/' => 44,
		'N' => 45,
		'M' => 46,
		'.' => 47,
		'`' => 50,
		_ => return Err(Error::UnsupportedKey(key.to_string())),
	});
	// TODO: create a `Virtual-Key Codes` table for Windows in `build.rs`.
	// Currently, we only support limited keys on Windows from:
	// https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes.
	#[cfg(target_os = "windows")]
	let k = Key::Other(match key {
		'0' => 0x30,
		'1' => 0x31,
		'2' => 0x32,
		'3' => 0x33,
		'4' => 0x34,
		'5' => 0x35,
		'6' => 0x36,
		'7' => 0x37,
		'8' => 0x38,
		'9' => 0x39,
		'A' => 0x41,
		'B' => 0x42,
		'C' => 0x43,
		'D' => 0x44,
		'E' => 0x45,
		'F' => 0x46,
		'G' => 0x47,
		'H' => 0x48,
		'I' => 0x49,
		'J' => 0x4A,
		'K' => 0x4B,
		'L' => 0x4C,
		'M' => 0x4D,
		'N' => 0x4E,
		'O' => 0x4F,
		'P' => 0x50,
		'Q' => 0x51,
		'R' => 0x52,
		'S' => 0x53,
		'T' => 0x54,
		'U' => 0x55,
		'V' => 0x56,
		'W' => 0x57,
		'X' => 0x58,
		'Y' => 0x59,
		'Z' => 0x5A,
		'-' => 0xBD,
		'=' => 0xBB,
		'[' => 0xDB,
		']' => 0xDD,
		'\\' => 0xDC,
		';' => 0xBA,
		'\'' => 0xDE,
		',' => 0xBC,
		'.' => 0xBE,
		'/' => 0xBF,
		'`' => 0xC0,
		_ => return Err(Error::UnsupportedKey(key.to_string())),
	});

	#[cfg(all(unix, not(target_os = "macos")))]
	let k = Key::Unicode(key);

	Ok(k)
}
