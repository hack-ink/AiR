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
use parking_lot::RwLock;

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

// TODO: next version.
#[allow(unused)]
pub struct Stated<T>(pub RwLock<T>)
where
	T: Clone;
#[allow(unused)]
impl<T> Stated<T>
where
	T: Clone,
{
	#[inline]
	pub fn try_write_setting(&self, setting: &mut T) {
		if let Some(v) = self.0.try_read() {
			v.clone_into(setting);
		}
	}

	#[inline]
	pub fn write_on_change(&self, response: Response, setting_value: T) {
		if response.changed() {
			*self.0.write() = setting_value;
		}
	}

	#[inline]
	pub fn write_on_lost_focus(&self, response: Response, setting_value: T) {
		if response.lost_focus() {
			*self.0.write() = setting_value;
		}
	}
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
