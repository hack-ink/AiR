// std
use std::{
	sync::mpsc::{self, Sender},
	thread,
};
// self
use crate::component::keyboard::Keyboard as Kb;

#[derive(Clone, Debug)]
pub struct Keyboard(Sender<Action>);
impl Keyboard {
	pub fn init() -> Self {
		let (tx, rx) = mpsc::channel::<Action>();

		// TODO: handle the error.
		thread::spawn(move || {
			let mut kb = Kb::init().unwrap();

			loop {
				let act = rx.recv().unwrap();

				tracing::info!("receive action: {act:?}");

				match act {
					Action::Copy => kb.copy().unwrap(),
					Action::Text(text) => kb.text(&text).unwrap(),
					Action::Abort => return,
				}
			}
		});

		Self(tx)
	}

	pub fn copy(&self) {
		self.0.send(Action::Copy).expect("send must succeed");
	}

	pub fn text(&self, text: String) {
		self.0.send(Action::Text(text)).expect("send must succeed");
	}

	pub fn abort(&self) {
		self.0.send(Action::Abort).expect("send must succeed");
	}
}

#[derive(Debug)]
enum Action {
	Copy,
	Text(String),
	Abort,
}
