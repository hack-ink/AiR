// std
use std::{
	sync::{atomic::Ordering, Once},
	time::Duration,
};
// crates.io
use eframe::{egui::*, Frame, *};
use tokio::runtime::Runtime;
// self
use crate::{
	component::Components,
	os::{AppKit, Os},
	prelude::Result,
	service::Services,
	ui::Uis,
};

#[derive(Debug)]
struct AiR {
	once: Once,
	runtime: Runtime,
	components: Components,
	services: Services,
	uis: Uis,
}
impl AiR {
	fn init(ctx: &Context) -> Self {
		Self::set_fonts(ctx);

		let once = Once::new();
		let runtime = Runtime::new().expect("runtime must be created");
		let components = Components::init();
		let services = Services::init(ctx);
		let uis = Uis::init();

		Self { once, runtime, components, services, uis }
	}

	fn set_fonts(ctx: &Context) {
		let mut fonts = FontDefinitions::default();

		// Cascadia Code.
		fonts.font_data.insert(
			"Cascadia Code".into(),
			FontData::from_static(include_bytes!("../asset/CascadiaCode.ttf")),
		);
		fonts
			.families
			.entry(FontFamily::Proportional)
			.or_default()
			.insert(0, "Cascadia Code".into());
		fonts.families.entry(FontFamily::Monospace).or_default().insert(0, "Cascadia Code".into());
		// NotoSerifSC.
		fonts.font_data.insert(
			"NotoSerifSC".into(),
			FontData::from_static(include_bytes!("../asset/NotoSerifSC-VariableFont_wght.ttf")),
		);
		fonts.families.entry(FontFamily::Proportional).or_default().insert(1, "NotoSerifSC".into());
		fonts.families.entry(FontFamily::Monospace).or_default().insert(1, "NotoSerifSC".into());

		ctx.set_fonts(fonts);
	}

	fn try_unhide(&mut self, ctx: &Context) {
		let to_hidden = self.services.to_hidden.load(Ordering::SeqCst);
		let focused = ctx.input(|i| i.focused);

		if to_hidden && !focused {
			self.components.active_timer.refresh();
			self.services.to_hidden.store(true, Ordering::SeqCst);

			// TODO: https://github.com/emilk/egui/discussions/4635.
			// ctx.send_viewport_cmd(ViewportCommand::Minimized(true));
			Os::hide();
		} else if !to_hidden && focused {
			// TODO: find a better place to initialize this.
			self.once.call_once(Os::set_move_to_active_space);
			self.services.to_hidden.store(true, Ordering::SeqCst);
		}
	}
}
impl App for AiR {
	fn update(&mut self, ctx: &Context, _: &mut Frame) {
		let air_ctx = AiRContext {
			egui_ctx: ctx,
			runtime: &self.runtime,
			components: &mut self.components,
			services: &mut self.services,
		};

		self.uis.draw(air_ctx);
		// TODO?: these will be called multiple times, move to focus service.
		self.try_unhide(ctx);

		if self.components.active_timer.duration() > Duration::from_secs(15) {
			self.components.active_timer.refresh();

			if self.uis.chat.input.is_empty() {
				self.components.quote.refresh(&self.runtime);
			}
		}
	}

	fn save(&mut self, _: &mut dyn Storage) {
		self.components.setting.save().expect("setting must be saved");
	}
}

#[derive(Debug)]
pub struct AiRContext<'a> {
	pub egui_ctx: &'a Context,
	pub runtime: &'a Runtime,
	pub components: &'a mut Components,
	pub services: &'a mut Services,
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
				.with_min_inner_size((720., 360.))
				.with_transparent(true),
			follow_system_theme: true,
			..Default::default()
		},
		Box::new(|c| Box::new(AiR::init(&c.egui_ctx))),
	)?;

	Ok(())
}
