// std
use std::{
	sync::{Arc, Mutex},
	thread,
	time::Duration,
};
// crates.io
use eframe::{
	egui::{CentralPanel, Context, ViewportBuilder},
	icon_data, App, CreationContext, Frame, NativeOptions, Storage,
};
// self
use crate::{component::util::Timer, data::*, os::*, prelude::*};

#[derive(Debug)]
struct AiR {
	init: Arc<Mutex<bool>>,
	active_timer: Timer,
	data: Data,
}
impl AiR {
	fn register_services(ctx: Context) -> Arc<Mutex<bool>> {
		let init = Arc::new(Mutex::new(true));
		let init_ = init.clone();

		// Give some time to all components to initialize themselves.
		thread::spawn(move || {
			loop {
				if !ctx.input(|i| i.focused) {
					thread::sleep(Duration::from_millis(10));
				} else {
					break;
				}
			}

			*init_.try_lock().unwrap() = false;
		});

		init
	}

	fn new(creation_ctx: &CreationContext) -> Self {
		Self {
			init: Self::register_services(creation_ctx.egui_ctx.clone()),
			active_timer: Timer::default(),
			data: Data::new(&creation_ctx.egui_ctx),
		}
	}
}
impl App for AiR {
	fn update(&mut self, ctx: &Context, _: &mut Frame) {
		CentralPanel::default().show(ctx, |ui| self.data.draw(ui));

		// TODO: these will be called multiple times.
		if !self.init.try_lock().map(|o| *o).unwrap_or(true) && !ctx.input(|i| i.focused) {
			self.active_timer.pause();

			Os::hide();
		}
		if self.active_timer.refresh() > Duration::from_secs(30) {
			self.active_timer.reset();
			// TODO: refactor `try_update`.
			self.data.refresh();
		}
	}

	fn save(&mut self, _: &mut dyn Storage) {
		self.data.save();
	}
}

pub fn launch() -> Result<()> {
	eframe::run_native(
		"AiR",
		NativeOptions {
			viewport: ViewportBuilder::default()
				.with_icon(
					icon_data::from_png_bytes(include_bytes!("../asset/icon.png").as_slice())
						.expect("icon must be valid"),
				)
				.with_inner_size((720., 360.))
				.with_min_inner_size((720., 360.)),
			..Default::default()
		},
		Box::new(|c| Box::new(AiR::new(c))),
	)?;

	Ok(())
}
