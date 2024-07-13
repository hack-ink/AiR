// std
use std::{
	fmt::{Debug, Formatter, Result as FmtResult},
	sync::mpsc::{self, Sender},
	thread,
};
// self
use crate::{component::audio::Audio as A, prelude::*};

#[derive(Clone)]
pub struct Audio(Sender<Effect>);
impl Audio {
	pub fn new() -> Result<Self> {
		let (tx, rx) = mpsc::channel::<Effect>();

		thread::spawn(move || {
			let audio = A::new().expect("audio must be created");

			loop {
				match rx.recv().expect("receive must succeed") {
					Effect::Notification => audio.play_notification(),
					Effect::Abort => return,
				}
			}
		});

		Ok(Self(tx))
	}

	pub fn play_notification(&self) {
		self.0.send(Effect::Notification).expect("send must succeed");
	}

	pub fn abort(&self) {
		let _ = self.0.send(Effect::Abort);
	}
}
impl Debug for Audio {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		write!(f, "Audio(..)")
	}
}

#[derive(Debug)]
enum Effect {
	Notification,
	Abort,
}
