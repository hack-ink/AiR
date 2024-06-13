mod hotkey;
use hotkey::Hotkey;

// TODO?: Init trait.

// std
use std::{
	sync::{
		atomic::{AtomicBool, Ordering},
		Arc,
	},
	thread,
	time::Duration,
};
// crates.io
use eframe::egui::Context;

#[derive(Debug)]
pub struct Services {
	// TODO: https://github.com/emilk/egui/issues/4468.
	pub to_hidden: Arc<AtomicBool>,
	pub hotkey: Hotkey,
}
impl Services {
	pub fn init(ctx: &Context) -> Self {
		let to_hidden = {
			let ctx: Context = ctx.to_owned();
			let to_hidden = Arc::new(AtomicBool::new(false));
			let to_hidden_ = to_hidden.clone();

			thread::spawn(move || {
				while !ctx.input(|i| i.focused) {
					thread::sleep(Duration::from_millis(50));
				}

				to_hidden_.store(false, Ordering::SeqCst);
			});

			to_hidden
		};
		let hotkey = Hotkey::init(ctx, to_hidden.clone());

		Self { to_hidden, hotkey }
	}
}
