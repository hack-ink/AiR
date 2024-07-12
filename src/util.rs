// crates.io
use eframe::egui::*;

pub fn price_rounded(value: f32) -> f32 {
	(value * 1_000_000.).round() / 1_000_000.
}

// TODO?: transparent window.
#[allow(unused)]
pub fn transparent_frame(ctx: &Context) -> Frame {
	Frame::central_panel(&ctx.style()).fill(Color32::TRANSPARENT)
}

pub fn modifiers_to_string(modifiers: &Modifiers) -> String {
	let mut s = String::new();

	if modifiers.ctrl {
		s.push_str("CTRL+");
	}
	if modifiers.shift {
		s.push_str("SHIFT+");
	}
	if modifiers.alt {
		s.push_str("ALT+");
	}
	if modifiers.command {
		s.push_str("META+");
	}

	s
}
