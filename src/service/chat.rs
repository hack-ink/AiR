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
};

pub type ChatArgs = (Function, String, bool);

#[derive(Debug)]
pub struct Chat {
	pub tx: Sender<ChatArgs>,
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
		let openai = Arc::new(Mutex::new(OpenAi::new(ai_setting.to_owned())));
		let openai_ = openai.clone();
		let chat_setting = Arc::new(Mutex::new(chat_setting.to_owned()));
		let chat_setting_ = chat_setting.clone();
		let input = state.input.clone();
		let output = state.output.clone();
		let tcs = state.token_counts.clone();
		let (tx, rx) = mpsc::channel();
		// TODO: handle the error.
		let abort_handle = rt
			.spawn(async move {
				loop {
					let (func, content, type_in): ChatArgs = rx.recv().unwrap();

					is_chatting.store(true, Ordering::Relaxed);

					tracing::info!("func: {func:?}");

					input.write().clone_from(&content);
					output.write().clear();

					let mut stream = openai_
						.lock()
						.await
						.chat(&func.prompt(&*chat_setting_.lock().await), &content)
						.await
						.unwrap();

					while let Some(r) = stream.next().await {
						let resp = r.unwrap();

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
				}
			})
			.abort_handle();

		Self { tx, openai, chat_setting, abort_handle }
	}

	pub fn send(&self, args: ChatArgs) {
		self.tx.send(args).expect("send must succeed");
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
