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
	// The manager need to be kept alive during the whole program life.
	ghk_manager: GlobalHotKeyManager,
	manager: Arc<RwLock<Manager>>,
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
		let _manager = GlobalHotKeyManager::new().map_err(GlobalHotKeyError::Main)?;
		let manager = Arc::new(RwLock::new(Manager::new(&_manager, hotkeys)));
		let manager_ = manager.clone();
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

					let (func, keys) = manager_.read().match_func(e.id);
					let to_focus = !func.is_directly();

					if to_focus {
						*activated_function.write() = func.basic();
						*focused_panel.write() = Panel::Chat;
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

		Ok(Self { ghk_manager: _manager, manager, abort })
	}

	pub fn renew(&self, hotkeys: &mut Hotkeys) {
		tracing::info!("renewing hotkey manager");

		let mut manager = self.manager.write();

		manager.unregister_hotkeys(&self.ghk_manager);

		*manager = Manager::new(&self.ghk_manager, hotkeys);

		// Write hotkey texts back into the settings.
		manager.read_hotkeys_into_settings(hotkeys);
	}

	pub fn abort(&self) {
		self.abort.store(true);
	}
}
impl Debug for Hotkey {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		f.debug_struct("Hotkey").field("manager", &"..").field("abort", &self.abort).finish()
	}
}

#[derive(Debug)]
struct HotKeyPair {
	hotkey: HotKey,
	keys: Keys,
}
struct Manager {
	/// Hotkeys. The order follows the order in settings
	hotkeys_list: [Option<HotKeyPair>; 4],
}
impl Manager {
	/// Creates new manager. Registers hotkeys with global manager.
	/// If any hotkey did not gersister, stores None instead of it.
	fn new(ghk_manager: &GlobalHotKeyManager, settings_hotkeys: &Hotkeys) -> Self {
		let hotkey_str_list = [
			&settings_hotkeys.rewrite,
			&settings_hotkeys.rewrite_directly,
			&settings_hotkeys.translate,
			&settings_hotkeys.translate_directly,
		];

		let mut hotkeys_list: Vec<Option<HotKeyPair>> = Vec::with_capacity(4);

		for hotkey_str in hotkey_str_list.into_iter() {
			//Parse error possible when str value is "Not set"
			let hotkey: HotKey = match hotkey_str.parse() {
				Ok(v) => v,
				Err(_) => {
					hotkeys_list.push(None);
					continue;
				},
			};
			// Same goes for Keys
			let keys: Keys = match hotkey_str.parse() {
				Ok(v) => v,
				Err(_) => {
					hotkeys_list.push(None);
					continue;
				},
			};
			// If manager.register fails, ignore the key, and it becomes "Not set".
			let e = ghk_manager.register(hotkey);
			if e.is_err() {
				hotkeys_list.push(None);
				continue;
			}

			// If key has been registered, add it to new_hotkeys
			hotkeys_list.push(Some(HotKeyPair { hotkey, keys }));
		}

		Self { hotkeys_list: hotkeys_list.try_into().expect("hotkeys_list must have 4 elements") }
	}

	fn match_func(&self, id: u32) -> (Function, Keys) {
		// Must follow the same order of functions as in settings
		const FUNCTION_LIST: [Function; 4] = [
			Function::Rewrite,
			Function::RewriteDirectly,
			Function::Translate,
			Function::TranslateDirectly,
		];

		for (i, pair) in self.hotkeys_list.iter().enumerate() {
			if let Some(pair) = pair {
				if pair.hotkey.id == id {
					return (FUNCTION_LIST[i], pair.keys.clone());
				}
			}
		}
		unreachable!();
	}

	// Copies text representation of actually registered hotkeys back to settings.
	// Replaces text for unset hotkeys with "Not set"
	fn read_hotkeys_into_settings(&self, settings_hotkeys: &mut Hotkeys) {
		let hotkey_str_list = [
			&mut settings_hotkeys.rewrite,
			&mut settings_hotkeys.rewrite_directly,
			&mut settings_hotkeys.translate,
			&mut settings_hotkeys.translate_directly,
		];

		for (i, k) in hotkey_str_list.into_iter().enumerate() {
			*k = match &self.hotkeys_list[i] {
				Some(p) => p.keys.to_string(),
				None => "Not set".to_string(),
			};
		}
	}

	/// Unregisters all hotkeys with given global hotkey manager.
	fn unregister_hotkeys(&mut self, ghk_manager: &GlobalHotKeyManager) {
		for list_entry in self.hotkeys_list.iter_mut() {
			if let Some(pair) = list_entry {
				ghk_manager.unregister(pair.hotkey).expect("unregister must succeed");
			}
			*list_entry = None;
		}
	}
}
