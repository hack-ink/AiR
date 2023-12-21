// std
use std::{
	sync::{Arc, Mutex},
	thread,
	time::Duration,
};
// crates.io
use eframe::{egui::*, Frame, *};
// air
use crate::{component::ActiveTimer, os::*, preference::*, ui::*};

#[derive(Debug)]
struct Air {
	on_initialize: Arc<Mutex<bool>>,
	active_timer: ActiveTimer,

	ui: Uii,
}
impl Air {
	fn set_hook(ctx: Context) -> Arc<Mutex<bool>> {
		let on_initialize = Arc::new(Mutex::new(true));
		let on_initialize_ = on_initialize.clone();

		// Give some time to all components to initialize themselves.
		thread::spawn(move || {
			loop {
				if !ctx.input(|i| i.focused) {
					thread::sleep(Duration::from_millis(10));
				} else {
					break;
				}
			}

			*on_initialize_.lock().unwrap() = false;
		});

		on_initialize
	}

	fn new(creation_ctx: &CreationContext) -> Self {
		Self {
			on_initialize: Self::set_hook(creation_ctx.egui_ctx.clone()),
			active_timer: ActiveTimer::new(),
			ui: Uii::new(&creation_ctx.egui_ctx),
		}
	}
}
impl App for Air {
	fn update(&mut self, ctx: &Context, _: &mut Frame) {
		CentralPanel::default().show(ctx, |ui| self.ui.draw(ui));

		if !*self.on_initialize.lock().unwrap() && !ctx.input(|i| i.focused) {
			// TODO: These will be called multiple times.

			self.active_timer.pause();

			Os::hide_application();
		}
		if self.active_timer.refresh() > Duration::from_secs(5) {
			self.active_timer.reset_timer();
			self.ui.try_update();
		}
	}
}

pub(super) fn launch() {
	PREFERENCE.set(Preference::default()).unwrap();

	eframe::run_native(
		"AIR",
		NativeOptions {
			run_and_return: true,
			viewport: ViewportBuilder::default()
				.with_min_inner_size((480., 320.))
				.with_inner_size((480., 320.)),
			..Default::default()
		},
		Box::new(|cc| Box::new(Air::new(cc))),
	)
	.unwrap();
}
