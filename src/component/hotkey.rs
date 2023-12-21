// std
use std::{
	sync::mpsc::{self, Receiver},
	thread,
	time::Duration,
};
// crates.io
use anyhow::Result;
use clipboard::*;
use global_hotkey::{hotkey::*, *};
// air
use crate::os::*;

#[derive(Debug)]
pub(crate) struct Hotkey(Receiver<Function>);
impl Hotkey {
	pub(crate) fn register() -> Result<Self> {
		let (tx, rx) = mpsc::channel();
		let manager = GlobalHotKeyManager::new()?;
		let receiver = GlobalHotKeyEvent::receiver();
		// Hotkeys.
		let hk_polish = HotKey::new(Some(Modifiers::CONTROL), Code::KeyU);
		let hk_polish_id = hk_polish.id();

		manager.register_all(&[hk_polish])?;

		thread::spawn(move || {
			// The manager need to be kept alive during the whole program life.
			let _manager = manager;
			let mut clipboard = ClipboardContext::new().unwrap();

			loop {
				if let Ok(e) = receiver.try_recv() {
					tracing::info!("{e:?}");

					if let HotKeyState::Pressed = e.state {
						if e.id == hk_polish_id {
							if let Some(t) = Os::get_selected_text() {
								let _ = clipboard.set_contents(t);
							}

							tx.send(Function::Polish).unwrap();

							Os::activate_application();
						}
					}
				}

				// Listening period.
				thread::sleep(Duration::from_millis(100));
			}
		});

		Ok(Self(rx))
	}

	pub(crate) fn try_recv(&self) -> Result<Function> {
		Ok(self.0.try_recv()?)
	}
}

#[derive(Debug)]
pub(crate) enum Function {
	Polish,
	// Translate,
}
