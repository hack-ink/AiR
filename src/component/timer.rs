// std
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Timer(Instant);
impl Timer {
	pub fn duration(&mut self) -> Duration {
		self.0.elapsed()
	}

	pub fn refresh(&mut self) {
		*self = Self::default();
	}
}
impl Default for Timer {
	fn default() -> Self {
		Self(Instant::now())
	}
}
