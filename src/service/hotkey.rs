// std
use std::{
	fmt::{Debug, Formatter, Result as FmtResult},
	sync::{
		atomic::{AtomicBool, Ordering},
		mpsc::{self, Receiver},
		Arc,
	},
	thread,
};
// crates.io
use arboard::Clipboard;
use eframe::egui::{Context, ViewportCommand};
use global_hotkey::{
	hotkey::{Code, HotKey, Modifiers},
	GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
};
// self
use crate::{component::function::Function, os::*};

pub struct Hotkey {
	pub clipboard: Clipboard,
	pub rx: Receiver<Function>,
}
impl Hotkey {
	pub fn init(ctx: &Context, to_hidden: Arc<AtomicBool>) -> Self {
		let ctx = ctx.to_owned();
		let clipboard = Clipboard::new().expect("clipboard must be available");
		let (tx, rx) = mpsc::channel();
		let manager = GlobalHotKeyManager::new().expect("hotkey manager must be created");
		let receiver = GlobalHotKeyEvent::receiver();
		// Hotkeys.
		let hk_polish = HotKey::new(Some(Modifiers::CONTROL), Code::KeyU);
		let hk_polish_id = hk_polish.id();

		manager.register_all(&[hk_polish]).expect("hotkey must be registered");

		thread::spawn(move || {
			// The manager need to be kept alive during the whole program life.
			let _manager = manager;
			let mut clipboard = Clipboard::new().expect("clipboard must be available");

			loop {
				// Block the thread until a hotkey event is received.
				match receiver.recv() {
					Ok(e) => {
						// We don't care about the release event.
						if let HotKeyState::Pressed = e.state {
							tracing::info!("receive hotkey event {e:?}");

							to_hidden.store(false, Ordering::SeqCst);
							// TODO: https://github.com/emilk/egui/discussions/4635.
							// ctx.send_viewport_cmd(ViewportCommand::Minimized(false));
							Os::unhide();

							match e.id {
								i if i == hk_polish_id => {
									if let Some(t) = Os::get_selected_text() {
										clipboard.set_text(t).expect("clipboard must be set");
									}

									tx.send(Function::Polish).expect("hotkey event must be sent");
								},
								_ => tracing::error!("unknown hotkey id {e:?}"),
							}

							// Give some time for the system to unhide and then focus.
							// Since `get_selected_text`` is slow, we don't need this for now.
							// thread::sleep(Duration::from_millis(50));

							ctx.send_viewport_cmd(ViewportCommand::Focus);
						}
					},
					Err(e) => panic!("failed to receive hotkey event {e:?}"),
				}
			}
		});

		Self { clipboard, rx }
	}

	pub fn try_recv(&mut self) -> Option<(Function, String)> {
		self.rx.try_recv().ok().map(|f| (f, self.clipboard.get_text().unwrap_or_default()))
	}
}
impl Debug for Hotkey {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		write!(f, "Hotkey {{ clipboard: .., rx: {:?} }}", self.rx)
	}
}
