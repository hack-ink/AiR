// std
use std::{
	sync::{Arc, RwLock},
	time::Duration,
};
// crates.io
use tokio::{runtime::Runtime, task::AbortHandle, time};
// self
use crate::component::quote::Quoter as QuoterC;

#[derive(Debug)]
pub struct Quoter {
	pub abort_handle: AbortHandle,
}
impl Quoter {
	pub fn init(rt: &Runtime, quote: Arc<RwLock<String>>) -> Self {
		let quoter = QuoterC;
		let abort_handle = rt
			.spawn(async move {
				loop {
					// TODO: skip if the chat input is not empty.

					let quote_ = quoter.fetch().await.unwrap_or(QuoterC::DEFAULT.into());

					if let Ok(mut quote) = quote.write() {
						*quote = quote_;
					} else {
						tracing::error!("quote got poisoned");

						return;
					}

					time::sleep(Duration::from_secs(30)).await;
				}
			})
			.abort_handle();

		Self { abort_handle }
	}

	pub fn abort(&self) {
		self.abort_handle.abort();
	}
}
