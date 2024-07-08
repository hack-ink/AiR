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
use parking_lot::RwLock;
use tokio::{runtime::Runtime, task::AbortHandle, time};
// self
use super::keyboard::Keyboard;
use crate::component::{
	function::Function,
	openai::OpenAi,
	setting::{Ai, Chat as ChatSetting},
};

pub type ChatArgs = (Function, String, bool);

#[derive(Debug)]
pub struct Chat {
	pub tx: Sender<ChatArgs>,
	abort_handle: AbortHandle,
}
impl Chat {
	pub fn new(
		keyboard: Keyboard,
		rt: &Runtime,
		is_chatting: Arc<AtomicBool>,
		ai_setting: Ai,
		chat_setting: ChatSetting,
		input: Arc<RwLock<String>>,
		output: Arc<RwLock<String>>,
	) -> Self {
		let openai = OpenAi::new(ai_setting);
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

					let mut stream =
						openai.chat(&func.prompt(&chat_setting), &content).await.unwrap();

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

		Self { abort_handle, tx }
	}

	pub fn abort(&self) {
		self.abort_handle.abort();
	}

	// TODO: fix clippy.
	#[allow(clippy::too_many_arguments)]
	pub fn renew(
		&mut self,
		keyboard: Keyboard,
		rt: &Runtime,
		is_chatting: Arc<AtomicBool>,
		ai_setting: Ai,
		chat_setting: ChatSetting,
		input: Arc<RwLock<String>>,
		output: Arc<RwLock<String>>,
	) {
		self.abort();

		*self = Self::new(keyboard, rt, is_chatting, ai_setting, chat_setting, input, output);
	}
}
