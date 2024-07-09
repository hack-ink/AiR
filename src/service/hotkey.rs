// std
use std::{
	sync::{
		atomic::{AtomicBool, Ordering},
		mpsc::Sender,
		Arc,
	},
	thread,
	time::Duration,
};
// crates.io
use arboard::Clipboard;
use eframe::egui::{Context, ViewportCommand};
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};
// self
use super::{audio::Audio, chat::ChatArgs, keyboard::Keyboard};
use crate::{
	component::{function::Function, keyboard::Keys, setting::Hotkeys},
	os::*,
	prelude::*,
};

#[derive(Debug)]
pub struct Hotkey(Arc<AtomicBool>);
impl Hotkey {
	pub fn new(
		ctx: &Context,
		keyboard: Keyboard,
		hotkeys: &Hotkeys,
		hide_on_lost_focus: Arc<AtomicBool>,
		audio: Audio,
		tx: Sender<ChatArgs>,
	) -> Result<Self> {
		let ctx = ctx.to_owned();
		let manager = Manager::new(hotkeys)?;
		let abort = Arc::new(AtomicBool::new(false));
		let abort_ = abort.clone();
		let hk_rx = GlobalHotKeyEvent::receiver();
		let mut clipboard = Clipboard::new()?;

		// TODO: handle the error.
		thread::spawn(move || {
			// Manager must be kept alive.
			let manager = manager;

			while !abort_.load(Ordering::Relaxed) {
				// Block the thread until a hotkey event is received.
				let e = hk_rx.recv().unwrap();

				// We don't care about the release event.
				if let HotKeyState::Pressed = e.state {
					audio.play_notification();

					let (func, keys) = manager.match_func(e.id);
					let to_focus = !func.is_directly();

					if to_focus && hide_on_lost_focus.load(Ordering::Relaxed) {
						Os::unhide();
					}

					// Reset the keys' state after the user triggers them.
					// If the user is still holding the keys, we can still perform the copy
					// operation successfully.
					keyboard.release_keys(keys);
					// Give system some time to response `releases_keys`.
					thread::sleep(Duration::from_millis(250));

					keyboard.copy();

					// Give some time to the system to refresh the clipboard.
					thread::sleep(Duration::from_millis(250));

					let content = match clipboard.get_text() {
						Ok(c) if !c.is_empty() => c,
						_ => continue,
					};

					tx.send((func, content, !to_focus)).expect("send must succeed");

					if to_focus {
						ctx.send_viewport_cmd(ViewportCommand::Focus);
					}
				}
			}
		});

		Ok(Self(abort))
	}

	// TODO: fn renew.

	pub fn abort(&self) {
		self.0.store(true, Ordering::Release);
	}
}

struct Manager {
	// The manager need to be kept alive during the whole program life.
	_inner: GlobalHotKeyManager,
	ids: [u32; 4],
	hotkeys_keys: [Keys; 4],
}
impl Manager {
	fn new(hotkeys: &Hotkeys) -> Result<Self> {
		let _inner = GlobalHotKeyManager::new().map_err(GlobalHotKeyError::Main)?;
		let hotkeys_raw = [
			&hotkeys.rewrite,
			&hotkeys.rewrite_directly,
			&hotkeys.translate,
			&hotkeys.translate_directly,
		];
		let hotkeys = hotkeys_raw
			.iter()
			.map(|h| h.parse())
			.collect::<Result<Vec<_>, _>>()
			.map_err(GlobalHotKeyError::Parse)?;

		_inner.register_all(&hotkeys).map_err(GlobalHotKeyError::Main)?;

		let ids =
			hotkeys.iter().map(|h| h.id).collect::<Vec<_>>().try_into().expect("array must fit");
		let hotkeys_keys = hotkeys_raw
			.iter()
			.map(|h| h.parse())
			.collect::<Result<Vec<_>, _>>()?
			.try_into()
			.expect("array must fit");

		Ok(Self { _inner, ids, hotkeys_keys })
	}

	fn match_func(&self, id: u32) -> (Function, Keys) {
		match id {
			i if i == self.ids[0] => (Function::Rewrite, self.hotkeys_keys[0].clone()),
			i if i == self.ids[1] => (Function::RewriteDirectly, self.hotkeys_keys[1].clone()),
			i if i == self.ids[2] => (Function::Translate, self.hotkeys_keys[2].clone()),
			i if i == self.ids[3] => (Function::TranslateDirectly, self.hotkeys_keys[3].clone()),
			_ => unreachable!(),
		}
	}
}
