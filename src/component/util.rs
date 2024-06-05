// std
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Timer {
	duration: Duration,
	instant: Option<Instant>,
}
impl Timer {
	pub fn refresh(&mut self) -> Duration {
		if let Some(i) = self.instant {
			self.duration + i.elapsed()
		} else {
			self.instant = Some(Instant::now());

			self.duration
		}
	}

	pub fn reset(&mut self) {
		*self = Self::default();
	}

	pub fn pause(&mut self) {
		self.duration = self.instant.map(|i| i.elapsed()).unwrap_or_default();
		self.instant = None;
	}
}
impl Default for Timer {
	fn default() -> Self {
		Self { duration: Default::default(), instant: Some(Instant::now()) }
	}
}

pub fn price_rounded(value: f32) -> f32 {
	(value * 1_000_000.).round() / 1_000_000.
}
