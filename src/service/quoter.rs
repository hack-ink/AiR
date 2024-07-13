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
	pub fn new(rt: &Runtime, quote: Arc<RwLock<String>>, input: Arc<RwLock<String>>) -> Self {
		*quote.write() = QuoterC::DEFAULT.into();

		let quoter = QuoterC;
		let abort_handle = rt
			.spawn(async move {
				loop {
					if input.read().is_empty() {
						if let Ok(quote_) = quoter.fetch().await {
							*quote.write() = quote_;
						}
					}

					time::sleep(Duration::from_millis(30_000)).await;
				}
			})
			.abort_handle();

		Self(abort_handle)
	}

	pub fn abort(&self) {
		self.0.abort();
	}
}
