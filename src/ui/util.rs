// crates.io
use eframe::egui::*;

pub fn transparent_frame(ctx: &Context) -> Frame {
	Frame::central_panel(&ctx.style()).fill(Color32::TRANSPARENT)
}
