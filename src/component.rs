mod fundamental;

pub(super) mod hotkey;
pub(super) mod quoter;

// std
use std::time::{Duration, Instant};

// TODO: Move to a single module.
#[derive(Debug)]
pub(crate) struct ActiveTimer {
	duration: Duration,
	instant: Option<Instant>,
}
impl ActiveTimer {
	pub(crate) fn new() -> Self {
		Self { duration: Default::default(), instant: Some(Instant::now()) }
	}

	pub(crate) fn refresh(&mut self) -> Duration {
		if let Some(i) = self.instant {
			self.duration + i.elapsed()
		} else {
			self.instant = Some(Instant::now());

			self.duration
		}
	}

	pub(crate) fn reset_timer(&mut self) {
		*self = Self::new();
	}

	pub(crate) fn pause(&mut self) {
		self.duration = self.instant.map(|i| i.elapsed()).unwrap_or_default();
		self.instant = None;
	}
}
