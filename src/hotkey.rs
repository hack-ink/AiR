// std
use std::{
	sync::{
		mpsc::{self, Receiver},
		Arc, Mutex,
	},
	thread,
	time::Duration,
};
// crates.io
use anyhow::Result;
use clipboard::{ClipboardContext, ClipboardProvider};
use global_hotkey::{
	hotkey::{Code, HotKey, Modifiers},
	GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
};
// air
use crate::os;

#[derive(Debug)]
pub(crate) enum Function {
	Polish,
	// Translate,
}

pub(crate) fn register(clipboard: Arc<Mutex<ClipboardContext>>) -> Result<Receiver<Function>> {
	let (hk_tx, hk_rx) = mpsc::channel();
	let manager = GlobalHotKeyManager::new()?;
	let receiver = GlobalHotKeyEvent::receiver();
	// Hotkeys.
	let hk_polish = HotKey::new(Some(Modifiers::CONTROL), Code::KeyU);
	let hk_polish_id = hk_polish.id();

	manager.register_all(&[hk_polish])?;

	thread::spawn(move || {
		// The manager need to be kept alive during the whole program life.
		let _manager = manager;

		loop {
			if let Ok(e) = receiver.try_recv() {
				tracing::info!("{e:?}");

				if let HotKeyState::Pressed = e.state {
					if e.id == hk_polish_id {
						if let Ok(mut c) = clipboard.try_lock() {
							if let Some(t) = os::get_selected_text() {
								let _ = c.set_contents(t);
							}
						}

						hk_tx.send(Function::Polish).unwrap();

						os::activate_application();
					}
				}
			}

			// Listening period.
			thread::sleep(Duration::from_millis(100));
		}
	});

	Ok(hk_rx)
}
