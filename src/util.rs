// std
use std::{
	fmt::Debug,
	sync::{
		atomic::{AtomicBool, Ordering},
		Arc,
	},
};
// crates.io
use eframe::egui::*;

macro_rules! impl_arts {
	($($n:ident, $t:ty, $i:ty,)+) => {
		$(
			#[derive(Clone, Debug, Default)]
			pub struct $n(pub Arc<$t>);
			impl $n {
				#[inline]
				pub fn new(value: $i) -> Self {
					Self(Arc::new(<$t>::new(value)))
				}

				#[inline]
				pub fn load(&self) -> $i {
					self.0.load(Ordering::Relaxed)
				}

				#[inline]
				pub fn store(&self, value: $i) {
					self.0.store(value, Ordering::Relaxed)
				}
			}
		)+
	};
}
impl_arts! {
	ArtBool, AtomicBool, bool,
}

pub fn unwrap_or_tracing<T, E>(result: Result<T, E>, tracing_prefix: &str) -> Option<T>
where
	E: Debug,
{
	match result {
		Ok(value) => Some(value),
		Err(ref e) => {
			tracing::error!("{tracing_prefix} due to: {e:?}");

			None
		},
	}
}

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
	if modifiers.mac_cmd {
		s.push_str("META+");
	}

	s
}
