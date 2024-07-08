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
	pub fn new() -> Self {
		let (tx, rx) = mpsc::channel::<Action>();

		// [`enigo::Enigo`] can't be sent between threads safely.
		// So, we spawn a new thread to handle the keyboard action here.
		thread::spawn(move || {
			let mut kb = Kb::new().expect("keyboard action must succeed");

			loop {
				match rx.recv().expect("receive must succeed") {
					Action::Copy => kb.copy().expect("keyboard action must succeed"),
					Action::Text(text) => kb.text(&text).expect("keyboard action must succeed"),
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
