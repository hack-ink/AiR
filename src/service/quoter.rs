// std
use std::{sync::Arc, time::Duration};
// crates.io
use parking_lot::RwLock;
use tokio::{runtime::Runtime, task::AbortHandle, time};
// self
use crate::component::quote::Quoter as QuoterC;

#[derive(Debug)]
pub struct Quoter(AbortHandle);
impl Quoter {
	pub fn new(rt: &Runtime, quote: Arc<RwLock<String>>) -> Self {
		let quoter = QuoterC;
		let abort_handle = rt
			.spawn(async move {
				loop {
					// TODO: skip if the chat input is not empty.

					*quote.write() = quoter.fetch().await.unwrap_or(QuoterC::DEFAULT.into());

					time::sleep(Duration::from_secs(50)).await;
				}
			})
			.abort_handle();

		Self(abort_handle)
	}

	pub fn abort(&self) {
		self.0.abort();
	}
}
