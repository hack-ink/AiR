// std
use std::sync::{
	atomic::{AtomicBool, Ordering},
	Arc,
};
// crates.io
use arboard::Clipboard;
use eframe::egui::{Context, ViewportCommand};
use futures::StreamExt;
use global_hotkey::{
	hotkey::{Code, HotKey, Modifiers},
	GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
};
use tokio::{
	runtime::{Handle, Runtime},
	task::{self, AbortHandle},
};
// self
use crate::{
	component::{function::Function, keyboard::Keyboard, Components},
	os::*,
	prelude::*,
	state::State,
};

#[derive(Debug)]
pub struct Hotkey {
	pub abort_handle: AbortHandle,
	pub is_running: Arc<AtomicBool>,
}
impl Hotkey {
	// TODO: optimize parameters.
	pub fn init(
		ctx: &Context,
		rt: &Runtime,
		components: &Components,
		state: &State,
	) -> Result<Self> {
		// The manager need to be kept alive during the whole program life.
		let (manager, hotkeys) = init_inner()?;
		let mut clipboard = Clipboard::new()?;
		let is_running = Arc::new(AtomicBool::new(false));
		let is_running_ = is_running.clone();
		let receiver = GlobalHotKeyEvent::receiver();
		let ctx = ctx.to_owned();
		let openai = components.openai.to_owned();
		let input = state.chat.input.to_owned();
		let output = state.chat.output.to_owned();
		let translation = state.setting.translation.to_owned();
		let abort_handle = rt
			.spawn(async move {
				// The manager need to be kept alive during the whole program life.
				let _manager = manager;

				loop {
					is_running_.store(false, Ordering::SeqCst);

					// Block the thread until a hotkey event is received.
					match receiver.recv() {
						Ok(e) => {
							// We don't care about the release event.
							if let HotKeyState::Pressed = e.state {
								tracing::info!("receive hotkey event {e:?}");

								is_running_.store(true, Ordering::SeqCst);

								let (func, to_get_selected_text) = match e.id {
									i if i == hotkeys[0] => (Function::Rewrite, true),
									i if i == hotkeys[1] => (Function::RewriteDirectly, true),
									i if i == hotkeys[2] => (Function::Translate, true),
									i if i == hotkeys[3] => (Function::TranslateDirectly, true),
									_ => unreachable!(),
								};
								let to_unhide = !matches!(
									func,
									Function::RewriteDirectly | Function::TranslateDirectly
								);

								if to_unhide {
									Os::unhide();
								}
								if to_get_selected_text {
									// TODO?: this might fail, not sure.
									if let Some(t) = Os::get_selected_text() {
										clipboard.set_text(t).unwrap();
									}
								}
								if to_unhide {
									// Give some time for the system to unhide and then focus.
									// Since `get_selected_text` is slow, we don't need this for
									// now.

									ctx.send_viewport_cmd(ViewportCommand::Focus);
								}

								let content = match clipboard.get_text() {
									Ok(c) if !c.is_empty() => c,
									_ => continue,
								};

								// TODO: handle the error.
								input.write().unwrap().clone_from(&content);
								output.write().unwrap().clear();

								// TODO: avoid clone.
								let output = output.clone();
								// TODO: sometimes, we don't need this.
								let translation = translation.read().unwrap().to_owned();

								// TODO?: task spawn.
								// TODO: handle the error.
								let mut stream = openai
									.lock()
									.await
									.chat(&func.prompt(&translation), &content)
									.await
									.unwrap();

								// TODO: handle the error.
								task::spawn_blocking(move || {
									// TODO: handle the error.
									// TODO: do not init this if not needed.
									let mut keyboard = Keyboard::init().unwrap();

									// TODO: handle the error.
									Handle::current()
										.block_on(async {
											while let Some(r) = stream.next().await {
												for s in r?
													.choices
													.into_iter()
													.filter_map(|c| c.delta.content)
												{
													// TODO?: handle the error.
													output.write().unwrap().push_str(&s);

													// TODO: move to outside of the loop.
													if matches!(
														func,
														Function::RewriteDirectly
															| Function::TranslateDirectly
													) {
														// TODO?: handle the error.
														keyboard.text(&s)?;
													}
												}
											}

											Ok::<_, Error>(())
										})
										.unwrap();
								})
								.await
								.unwrap();
							}
						},
						Err(e) => panic!("failed to receive hotkey event {e:?}"),
					}
				}
			})
			.abort_handle();

		Ok(Self { abort_handle, is_running })
	}

	pub fn abort(&self) {
		self.abort_handle.abort();
	}

	pub fn is_running(&self) -> bool {
		self.is_running.load(Ordering::SeqCst)
	}
}

// TODO: make into a struct.
fn init_inner() -> Result<(GlobalHotKeyManager, [u32; 4])> {
	let manager = GlobalHotKeyManager::new()?;
	let hk_rewrite = HotKey::new(Some(Modifiers::CONTROL), Code::KeyT);
	let hk_rewrite_id = hk_rewrite.id();
	let hk_rewrite_directly = HotKey::new(Some(Modifiers::CONTROL), Code::KeyY);
	let hk_rewrite_directly_id = hk_rewrite_directly.id();
	let hk_translate = HotKey::new(Some(Modifiers::CONTROL), Code::KeyU);
	let hk_translate_id = hk_translate.id();
	let hk_translate_directly = HotKey::new(Some(Modifiers::CONTROL), Code::KeyI);
	let hk_translate_directly_id = hk_translate_directly.id();

	manager.register_all(&[
		hk_rewrite,
		hk_rewrite_directly,
		hk_translate,
		hk_translate_directly,
	])?;

	Ok((
		manager,
		[hk_rewrite_id, hk_rewrite_directly_id, hk_translate_id, hk_translate_directly_id],
	))
}
