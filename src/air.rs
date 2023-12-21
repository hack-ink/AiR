// std
use std::{
	sync::{Arc, Mutex},
	thread,
	time::Duration,
};
// crates.io
use eframe::{egui::*, Frame, *};
use ureq::{Agent, AgentBuilder};
// air
use crate::{os::*, preference::*, ui::*};

#[derive(Debug)]
struct Air {
	on_initialize: Arc<Mutex<bool>>,
	ui: Uii,
	http: Agent,
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
			ui: Uii::new(&creation_ctx.egui_ctx),
			http: AgentBuilder::new()
				.timeout_read(Duration::from_secs(5))
				.timeout_write(Duration::from_secs(5))
				.build(),
		}
	}
}
impl App for Air {
	fn update(&mut self, ctx: &Context, _: &mut Frame) {
		if !*self.on_initialize.lock().unwrap() && !ctx.input(|i| i.focused) {
			// TODO: This will be called multiple times.
			Os::hide_application();
		}

		CentralPanel::default().show(ctx, |ui| self.ui.draw(ui));
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
