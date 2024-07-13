// std
use std::sync::Once;
// crates.io
use eframe::{egui::*, glow::Context as GlowContext, Frame, *};
use tracing_subscriber::{reload::Handle, EnvFilter, Registry};
// self
use crate::{
	component::{os::Os, Components},
	prelude::Result,
	service::Services,
	state::State,
	ui::{self, Uis},
};

#[derive(Debug)]
struct AiR {
	once: Once,
	components: Components,
	state: State,
	services: Services,
	uis: Uis,
}
impl AiR {
	fn new(log_filter_handle: Handle<EnvFilter, Registry>, ctx: &Context) -> Result<Self> {
		ui::set_fonts(ctx);

		// To enable SVG.
		egui_extras::install_image_loaders(ctx);

		let once = Once::new();
		let components = Components::new()?;

		ui::set_font_size(ctx, components.setting.general.font_size);

		let state = State::new(log_filter_handle, &components.setting)?;
		let services = Services::new(ctx, &components, &state)?;
		let uis = Uis::new();

		Ok(Self { once, components, state, services, uis })
	}
}
impl App for AiR {
	fn update(&mut self, ctx: &Context, _: &mut Frame) {
		self.uis.draw(AiRContext {
			egui_ctx: ctx,
			components: &mut self.components,
			state: &self.state,
			services: &mut self.services,
		});
	}

	fn save(&mut self, _: &mut dyn Storage) {
		self.components.setting.save().unwrap();
	}

	fn raw_input_hook(&mut self, _: &Context, raw_input: &mut RawInput) {
		if let Some(focused) = raw_input.events.iter().find_map(|e| {
			if let Event::WindowFocused(focused) = e {
				Some(*focused)
			} else {
				None
			}
		}) {
			if focused {
				// This must be called on the main thread and after the window fully get
				// initialized.
				//
				// If possible find a place to call this only once.
				self.once.call_once(Os::set_move_to_active_space);
			}
			// TODO: https://github.com/emilk/egui/issues/4468.
			// Allow 250ms for initialization during the first boot.
			else if raw_input.time.unwrap_or_default() >= 0.25
				&& self.components.setting.general.hide_on_lost_focus
			{
				// TODO: https://github.com/emilk/egui/discussions/4635.
				// We can get rid of the OS API if this works.
				// ctx.send_viewport_cmd(ViewportCommand::Minimized(true));
				self.components.os.hide();
			}
		}
	}

	fn on_exit(&mut self, _: Option<&GlowContext>) {
		self.services.abort();
	}
}

#[derive(Debug)]
pub struct AiRContext<'a> {
	pub egui_ctx: &'a Context,
	pub components: &'a mut Components,
	pub state: &'a State,
	pub services: &'a mut Services,
}

pub fn launch(log_filter_handle: Handle<EnvFilter, Registry>) -> Result<()> {
	eframe::run_native(
		"AiR",
		NativeOptions {
			viewport: ViewportBuilder::default()
				.with_icon(
					icon_data::from_png_bytes(include_bytes!("../asset/icon.png").as_slice())
						.unwrap(),
				)
				.with_inner_size((760., 360.))
				.with_min_inner_size((760., 360.)),
			// TODO?: transparent window.
			// .with_transparent(true),
			follow_system_theme: true,
			..Default::default()
		},
		Box::new(|c| Ok(Box::new(AiR::new(log_filter_handle, &c.egui_ctx).unwrap()))),
	)?;

	Ok(())
}
