// std
use std::{sync::mpsc::Sender, time::Duration};
// crates.io
use arboard::Clipboard;
use eframe::egui::{Context, ViewportCommand};
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};
use tokio::{runtime::Runtime, task::AbortHandle, time};
// self
use super::{chat::ChatArgs, keyboard::Keyboard};
use crate::{
	component::{function::Function, setting::Hotkeys},
	os::*,
	prelude::*,
};

#[derive(Debug)]
pub struct Hotkey(AbortHandle);
impl Hotkey {
	pub fn new(
		ctx: &Context,
		keyboard: Keyboard,
		rt: &Runtime,
		hotkeys: &Hotkeys,
		tx: Sender<ChatArgs>,
	) -> Result<Self> {
		let ctx = ctx.to_owned();
		let manager = Manager::new(hotkeys)?;
		let receiver = GlobalHotKeyEvent::receiver();
		let mut clipboard = Clipboard::new()?;
		// TODO: handle the error.
		let abort_handle = rt
			.spawn(async move {
				// The manager need to be kept alive during the whole program life.
				let manager = manager;

				loop {
					// Block the thread until a hotkey event is received.
					let e = receiver.recv().unwrap();

					// We don't care about the release event.
					if let HotKeyState::Pressed = e.state {
						// TODO: reset the hotkey state so that we don't need to wait for the user
						// to release the keys.

						let func = manager.match_func(e.id);
						let to_unhide = !func.is_directly();

						if to_unhide {
							Os::unhide();
						}

						// Sleep for a while to reset the keyboard state after user
						// triggers the hotkey.
						time::sleep(Duration::from_millis(1000)).await;

						keyboard.copy();

						// Give some time to the system to refresh the clipboard.
						time::sleep(Duration::from_millis(500)).await;

						let content = match clipboard.get_text() {
							Ok(c) if !c.is_empty() => c,
							_ => continue,
						};

						tx.send((func, content, !to_unhide)).unwrap();

						if to_unhide {
							// Generally, this needs some time to wait the window available
							// first, but the previous sleep in get selected text is enough.
							ctx.send_viewport_cmd(ViewportCommand::Focus);
						}
					}
				}
			})
			.abort_handle();

		Ok(Self(abort_handle))
	}

	pub fn abort(&self) {
		self.0.abort();
	}

	// TODO: fn renew.
}

struct Manager {
	// The manager need to be kept alive during the whole program life.
	_inner: GlobalHotKeyManager,
	ids: [u32; 4],
}
impl Manager {
	fn new(hotkeys: &Hotkeys) -> Result<Self> {
		let _inner = GlobalHotKeyManager::new()?;
		let hotkeys = [
			hotkeys.rewrite,
			hotkeys.rewrite_directly,
			hotkeys.translate,
			hotkeys.translate_directly,
		];

		_inner.register_all(&hotkeys)?;

		let ids = hotkeys.iter().map(|h| h.id).collect::<Vec<_>>().try_into().unwrap();

		Ok(Self { _inner, ids })
	}

	fn match_func(&self, id: u32) -> Function {
		match id {
			i if i == self.ids[0] => Function::Rewrite,
			i if i == self.ids[1] => Function::RewriteDirectly,
			i if i == self.ids[2] => Function::Translate,
			i if i == self.ids[3] => Function::TranslateDirectly,
			_ => unreachable!(),
		}
	}
}
