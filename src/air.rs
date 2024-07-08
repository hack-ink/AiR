// std
use std::sync::Once;
// crates.io
use eframe::{egui::*, glow::Context as GlowContext, Frame, *};
// self
use crate::{
	component::Components, os::Os, prelude::Result, service::Services, state::State, ui::Uis,
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
	fn new(ctx: &Context) -> Result<Self> {
		Self::set_fonts(ctx);

		// To enable SVG.
		egui_extras::install_image_loaders(ctx);

		let once = Once::new();
		let components = Components::new()?;
		let state = Default::default();
		let services = Services::new(ctx, &components, &state)?;
		let uis = Uis::new();

		Ok(Self { once, components, state, services, uis })
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
}
impl App for AiR {
	fn update(&mut self, ctx: &Context, _: &mut Frame) {
		let air_ctx = AiRContext {
			egui_ctx: ctx,
			components: &mut self.components,
			state: &self.state,
			services: &mut self.services,
		};

		self.uis.draw(air_ctx);
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
			// Allow 1 second for initialization during the first boot.
			else if raw_input.time.unwrap_or_default() >= 1. {
				// TODO: https://github.com/emilk/egui/discussions/4635.
				// We can get rid of the OS API if this works.
				// ctx.send_viewport_cmd(ViewportCommand::Minimized(false));
				Os::hide();
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

pub fn launch() -> Result<()> {
	eframe::run_native(
		"AiR",
		NativeOptions {
			viewport: ViewportBuilder::default()
				.with_icon(
					icon_data::from_png_bytes(include_bytes!("../asset/icon.png").as_slice())
						.unwrap(),
				)
				.with_inner_size((720., 360.))
				.with_min_inner_size((720., 360.))
				.with_transparent(true),
			..Default::default()
		},
		Box::new(|c| Ok(Box::new(AiR::new(&c.egui_ctx).unwrap()))),
	)?;

	Ok(())
}
