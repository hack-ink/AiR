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
use parking_lot::{RwLock, RwLockWriteGuard};

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

// Make easier easier to state an existing value.
//
// This is useful when you need to instantiate a value and cache it simultaneously.
#[derive(Clone, Debug, Default)]
pub struct Stated<T>(pub Arc<RwLock<T>>)
where
	T: Clone;
#[allow(unused)]
impl<T> Stated<T>
where
	T: Clone,
{
	#[inline]
	pub fn new(value: T) -> Self {
		Self(Arc::new(RwLock::new(value)))
	}

	#[inline]
	pub fn inner(&self) -> Arc<RwLock<T>> {
		self.0.clone()
	}

	#[inline]
	pub fn write(&self) -> RwLockWriteGuard<T> {
		self.0.write()
	}

	#[inline]
	pub fn set(&self, value: T) {
		*self.write() = value;
	}

	#[inline]
	pub fn sync_from(&self, source: &T) {
		self.write().clone_from(source);
	}

	// This must be done before drawing the UI.
	#[inline]
	pub fn try_sync_to(&self, target: &mut T) {
		if let Some(v) = self.0.try_read() {
			v.clone_into(target);
		}
	}

	// Suggest to place this after drawing the UI.
	#[inline]
	pub fn sync_on_change(&self, response: Response, target: T) {
		if response.changed() {
			*self.write() = target;
		}
	}

	// Suggest to place this after drawing the UI.
	#[inline]
	pub fn sync_on_lost_focus(&self, response: Response, target: T) {
		if response.lost_focus() {
			*self.write() = target;
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
