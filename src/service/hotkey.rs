// std
use std::{
	fmt::{Debug, Formatter, Result as FmtResult},
	sync::{mpsc::Sender, Arc},
	thread,
	time::Duration,
};
// crates.io
use arboard::Clipboard;
use eframe::egui::{Context, ViewportCommand};
use global_hotkey::{hotkey::HotKey, GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};
use parking_lot::RwLock;
// self
use super::{audio::Audio, chat::ChatArgs, keyboard::Keyboard};
use crate::{
	component::{function::Function, keyboard::Keys, os::Os, setting::Hotkeys},
	prelude::*,
	state::State,
	ui::panel::Panel,
	util::ArtBool,
};

pub struct Hotkey {
	// The global hokey manager need to be kept alive during the whole program life.
	ghk: GlobalHotKeyManager,
	// TODO: https://github.com/hack-ink/AiR/issues/117.
	inner: Arc<RwLock<Manager>>,
	abort: ArtBool,
}
impl Hotkey {
	pub fn new(
		ctx: &Context,
		hotkeys: &Hotkeys,
		state: &State,
		keyboard: Keyboard,
		audio: Audio,
		tx: Sender<ChatArgs>,
	) -> Result<Self> {
		let ctx = ctx.to_owned();
		let ghk = GlobalHotKeyManager::new().map_err(GlobalHotKeyError::Main)?;
		let inner = Arc::new(RwLock::new(Manager::new(&ghk, hotkeys)?));
		let inner_ = inner.clone();
		let notification_sound = state.general.notification_sound.clone();
		let activated_function = state.chat.activated_function.clone();
		let focused_panel = state.ui.focused_panel.clone();
		let abort = ArtBool::new(false);
		let abort_ = abort.clone();
		let hk_rx = GlobalHotKeyEvent::receiver();
		let mut clipboard = Clipboard::new()?;

		// TODO: handle the error.
		thread::spawn(move || {
			// Only Windows needs to obtain the window handle.
			#[cfg(target_os = "windows")]
			let os = {
				let mut os = Os::new();

				os.obtain_window();

				os
			};
			#[cfg(not(target_os = "windows"))]
			let os = Os::new();

			while !abort_.load() {
				// Block the thread until a hotkey event is received.
				let e = hk_rx.recv().unwrap();

				// We don't care about the release event.
				if let HotKeyState::Pressed = e.state {
					if notification_sound.load() {
						audio.play_notification();
					}

					let (func, keys) = inner_.read().match_function(e.id);
					let to_focus = !func.is_directly();

					if to_focus {
						activated_function.set(func.basic());
						focused_panel.set(Panel::Chat);

						// TODO: check if the window is hidden.
						os.unhide();
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

		Ok(Self { ghk, inner, abort })
	}

	pub fn renew(&self, hotkeys: &Hotkeys) -> Result<()> {
		tracing::info!("renewing hotkey manager");

		for (hk, _) in self.inner.read().0.iter().filter_map(|maybe_hk| maybe_hk.as_ref()) {
			self.ghk.unregister(*hk).map_err(GlobalHotKeyError::Main)?;
		}

		*self.inner.write() = Manager::new(&self.ghk, hotkeys)?;

		Ok(())
	}

	pub fn abort(&self) {
		self.abort.store(true);
	}
}
impl Debug for Hotkey {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		f.debug_struct("Hotkey")
			.field("ghk", &"..")
			.field("inner", &self.inner)
			.field("abort", &self.abort)
			.finish()
	}
}

// The order of `[(HotKey, Keys); 4]` must
// match that of the corresponding function in `[Function::all()]`.
#[derive(Debug)]
struct Manager([Option<(HotKey, Keys)>; 4]);
impl Manager {
	fn new(ghk: &GlobalHotKeyManager, hotkeys: &Hotkeys) -> Result<Self> {
		let hotkeys_raw = [
			&hotkeys.rewrite,
			&hotkeys.rewrite_directly,
			&hotkeys.translate,
			&hotkeys.translate_directly,
		];
		let hotkeys = hotkeys_raw
			.iter()
			.map(|maybe_hk| {
				let hk = maybe_hk.validate()?;

				if let Some((hk, _)) = &hk {
					ghk.register(*hk).map_err(GlobalHotKeyError::Main)?;
				}

				Ok(hk)
			})
			.collect::<Result<Vec<_>>>()?
			.try_into()
			.expect("array must fit");

		Ok(Self(hotkeys))
	}

	fn match_function(&self, id: u32) -> (Function, Keys) {
		for f in Function::all() {
			if let Some((hk, ks)) = &self.0[f as usize] {
				if hk.id() == id {
					return (f, ks.clone());
				}
			}
		}

		unreachable!()
	}
}
