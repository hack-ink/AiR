// std
use std::{
	sync::{
		atomic::{AtomicBool, Ordering},
		mpsc::{self, Sender},
		Arc,
	},
	time::Duration,
};
// crates.io
use futures::StreamExt;
use tokio::{runtime::Runtime, sync::Mutex, task::AbortHandle, time};
// self
use super::keyboard::Keyboard;
use crate::{
	component::{
		function::Function,
		openai::OpenAi,
		setting::{Ai, Chat as ChatSetting},
	},
	state::Chat as ChatState,
	util,
};

pub type ChatArgs = (Function, String, bool);

#[derive(Debug)]
pub struct Chat {
	pub tx: Sender<ChatArgs>,
	to_interrupt: Arc<AtomicBool>,
	// TODO?: get rid of the `Mutex`.
	openai: Arc<Mutex<OpenAi>>,
	chat_setting: Arc<Mutex<ChatSetting>>,
	abort_handle: AbortHandle,
}
impl Chat {
	pub fn new(
		keyboard: Keyboard,
		rt: &Runtime,
		is_chatting: Arc<AtomicBool>,
		ai_setting: &Ai,
		chat_setting: &ChatSetting,
		state: &ChatState,
	) -> Self {
		let (tx, rx) = mpsc::channel();
		let to_interrupt = Arc::new(AtomicBool::new(false));
		let to_interrupt_ = to_interrupt.clone();
		let openai = Arc::new(Mutex::new(OpenAi::new(ai_setting.to_owned())));
		let openai_ = openai.clone();
		let chat_setting = Arc::new(Mutex::new(chat_setting.to_owned()));
		let chat_setting_ = chat_setting.clone();
		let input = state.input.clone();
		let output = state.output.clone();
		let tcs = state.token_counts.clone();
		let error = state.error.clone();
		// TODO: handle the error.
		let abort_handle = rt
			.spawn(async move {
				'listen: loop {
					let (func, content, type_in): ChatArgs = rx.recv().unwrap();

					is_chatting.store(true, Ordering::Relaxed);

					tracing::info!("func: {func:?}");

					input.write().clone_from(&content);
					output.write().clear();

					let Some(mut stream) = util::unwrap_or_tracing(
						openai_
							.lock()
							.await
							.chat(&func.prompt(&*chat_setting_.lock().await), &content)
							.await,
						"failed to create the chat stream",
					) else {
						is_chatting.store(false, Ordering::Relaxed);
						error.store(true, Ordering::Relaxed);

						continue;
					};

					while let Some(r) = stream.next().await {
						if to_interrupt_.load(Ordering::Relaxed) {
							to_interrupt_.store(false, Ordering::Relaxed);
							is_chatting.store(false, Ordering::Relaxed);

							continue 'listen;
						}

						let Some(resp) = util::unwrap_or_tracing(
							r,
							"failed to retrieve the next item from the stream",
						) else {
							is_chatting.store(false, Ordering::Relaxed);
							error.store(true, Ordering::Relaxed);

							continue 'listen;
						};

						for s in resp.choices.into_iter().filter_map(|c| c.delta.content) {
							output.write().push_str(&s);

							// TODO?: move to outside of the loop.
							if type_in {
								keyboard.text(s);
							}
						}

						if let Some(u) = resp.usage {
							tcs.0.store(u.prompt_tokens, Ordering::Relaxed);
							tcs.1.store(u.completion_tokens, Ordering::Relaxed);
						}
					}

					// Allow the UI a moment to refresh the content.
					time::sleep(Duration::from_millis(50)).await;

					is_chatting.store(false, Ordering::Relaxed);
					error.store(false, Ordering::Relaxed);
				}
			})
			.abort_handle();

		Self { tx, to_interrupt, openai, chat_setting, abort_handle }
	}

	pub fn send(&self, args: ChatArgs) {
		self.tx.send(args).expect("send must succeed");
	}

	pub fn interrupt(&self) {
		self.to_interrupt.store(true, Ordering::Relaxed);
	}

	pub fn renew(&self, ai_setting: &Ai, chat_setting: &ChatSetting) {
		tracing::info!("renewing openai client");

		*self.openai.blocking_lock() = OpenAi::new(ai_setting.to_owned());

		chat_setting.clone_into(&mut self.chat_setting.blocking_lock());
	}

	pub fn abort(&self) {
		self.abort_handle.abort();
	}
}
