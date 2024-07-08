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
		setting::{Chat as ChatSetting, Setting},
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
		setting: &Setting,
		state: &ChatState,
	) -> Self {
		let openai = Arc::new(Mutex::new(OpenAi::new(setting.ai.clone())));
		let openai_ = openai.clone();
		let chat_setting = Arc::new(Mutex::new(setting.chat.clone()));
		let chat_setting_ = chat_setting.clone();
		let input = state.input.clone();
		let output = state.output.clone();
		let (tx, rx) = mpsc::channel();
		// TODO: handle the error.
		let abort_handle = rt
			.spawn(async move {
				loop {
					let (func, content, type_in): ChatArgs = rx.recv().unwrap();

					is_chatting.store(true, Ordering::SeqCst);

					tracing::info!("func: {func:?}");
					tracing::debug!("content: {content}");

					input.write().clone_from(&content);
					output.write().clear();

					let mut stream = openai_
						.lock()
						.await
						.chat(&func.prompt(&*chat_setting_.lock().await), &content)
						.await
						.unwrap();

					while let Some(r) = stream.next().await {
						for s in r.unwrap().choices.into_iter().filter_map(|c| c.delta.content) {
							output.write().push_str(&s);

							// TODO?: move to outside of the loop.
							if type_in {
								keyboard.text(s);
							}
						}
					}

					// Allow the UI a moment to refresh the content.
					time::sleep(Duration::from_millis(50)).await;

					is_chatting.store(false, Ordering::SeqCst);
				}
			})
			.abort_handle();

		Self { tx, openai, chat_setting, abort_handle }
	}

	pub fn send(&self, args: ChatArgs) {
		self.tx.send(args).expect("send must succeed");
	}

	pub fn renew(&mut self, setting: &Setting) {
		*self.openai.blocking_lock() = OpenAi::new(setting.ai.clone());
		*self.chat_setting.blocking_lock() = setting.chat.clone();
	}

	pub fn abort(&self) {
		self.abort_handle.abort();
	}
}
