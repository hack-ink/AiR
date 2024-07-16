// std
use std::borrow::Cow;
// crates.io
use eframe::egui::{self, *};
use language::Language;
// self
use crate::util;

pub const ICON_PIXELS: f32 = 16.;
pub const SMALL_FONT_OFFSET: f32 = 3.;

pub trait ComboBoxItem
where
	Self: Sized + Clone + PartialEq,
{
	const COUNT: usize;

	type Array: AsRef<[Self]>;

	// TODO: Rust doesn't support generic const yet.
	// `[Self; Self::COUNT]` is not allowed.
	fn all() -> Self::Array;

	fn display(&self) -> Cow<str>;
}

#[derive(Debug, Default)]
pub struct ShortcutWidget {
	pub send: SendWidget,
	pub interrupt: InterruptWidget,
	pub copy: CopyWidget,
}
#[derive(Debug)]
pub struct SendWidget {
	pub send_icon_d: Image<'static>,
	pub send_icon_l: Image<'static>,
}
impl SendWidget {
	pub fn icon(&self, dark_mode: bool) -> Image<'static> {
		if dark_mode {
			self.send_icon_d.clone()
		} else {
			self.send_icon_l.clone()
		}
	}
}
impl Default for SendWidget {
	fn default() -> Self {
		Self {
			send_icon_d: image_button(include_image!("../asset/send-dark.svg"), ICON_PIXELS),
			send_icon_l: image_button(include_image!("../asset/send-light.svg"), ICON_PIXELS),
		}
	}
}
#[derive(Debug)]
pub struct InterruptWidget {
	pub interrupt_icon_d: Image<'static>,
	pub interrupt: Image<'static>,
}
impl InterruptWidget {
	pub fn icon(&self, dark_mode: bool) -> Image<'static> {
		if dark_mode {
			self.interrupt_icon_d.clone()
		} else {
			self.interrupt.clone()
		}
	}
}
impl Default for InterruptWidget {
	fn default() -> Self {
		Self {
			interrupt_icon_d: image_button(
				include_image!("../asset/interrupt-dark.svg"),
				ICON_PIXELS,
			),
			interrupt: image_button(include_image!("../asset/interrupt-light.svg"), ICON_PIXELS),
		}
	}
}
#[derive(Debug)]
pub struct CopyWidget {
	pub copy_icon_l: Image<'static>,
	pub copy_icon_d: Image<'static>,
	pub copied_icon_l: Image<'static>,
	pub copied_icon_d: Image<'static>,
	pub triggered: bool,
}
impl CopyWidget {
	pub fn copy_icon(&self, dark_mode: bool) -> Image<'static> {
		if dark_mode {
			self.copy_icon_d.clone()
		} else {
			self.copy_icon_l.clone()
		}
	}

	pub fn copied_icon(&self, dark_mode: bool) -> Image<'static> {
		if dark_mode {
			self.copied_icon_d.clone()
		} else {
			self.copied_icon_l.clone()
		}
	}
}
impl Default for CopyWidget {
	fn default() -> Self {
		Self {
			copy_icon_d: image_button(include_image!("../asset/copy-dark.svg"), ICON_PIXELS),
			copy_icon_l: image_button(include_image!("../asset/copy-light.svg"), ICON_PIXELS),
			copied_icon_d: image_button(include_image!("../asset/copied-dark.svg"), ICON_PIXELS),
			copied_icon_l: image_button(include_image!("../asset/copied-light.svg"), ICON_PIXELS),
			triggered: false,
		}
	}
}

#[derive(Debug, Default)]
pub struct HotkeyListener {
	listening: bool,
}
impl HotkeyListener {
	pub fn listen(&mut self, ui: &mut Ui, label: &str, hotkey: &mut String) -> bool {
		ui.label(label);

		let text = if self.listening { "Press Desired Key Combination" } else { hotkey.as_str() };
		let resp = ui.add(Label::new(text).selectable(false).sense(Sense::click()));
		let mut changed = false;

		if resp.clicked() {
			self.listening = true;
		}
		if self.listening {
			ui.input(|i| {
				let mut abort = false;

				for e in &i.events {
					if let Event::Key { key, pressed, modifiers, .. } = e {
						if *pressed {
							// TODO?: do we allow a single key here.
							*hotkey = format!("{}{key:?}", util::modifiers_to_string(modifiers));
							changed = true;
							abort = true;

							break;
						}
					}
					if let Event::PointerButton { pressed, .. } = e {
						abort = *pressed;
					}

					if matches!(e, Event::WindowFocused(..)) {
						abort = true;
					}
				}

				if abort {
					self.listening = false;
				}
			});
		}

		changed
	}
}

impl ComboBoxItem for Language {
	type Array = [Language; Self::COUNT];

	const COUNT: usize = 250;

	fn all() -> Self::Array {
		Language::all()
	}

	fn display(&self) -> Cow<str> {
		Cow::Owned(format!("{} {}", self.as_tag(), self.as_local()))
	}
}

pub fn combo_box<'a, I>(label: &'a str, current: &'a mut I) -> impl Widget + 'a
where
	I: Clone + PartialEq + ComboBoxItem,
{
	move |ui: &mut Ui| {
		ui.label(label);

		let mut resp =
			ComboBox::from_id_source(label).selected_text(current.display()).show_ui(ui, |ui| {
				I::all().as_ref().iter().fold(false, |changed, i| {
					changed | ui.selectable_value(current, i.to_owned(), i.display()).changed()
				})
			});

		if let Some(changed) = resp.inner {
			if changed {
				resp.response.mark_changed();
			}
		}

		resp.response
	}
}

// https://github.com/emilk/egui/blob/aa96b257460a07b30489d104fae08d095a9e3a4e/crates/egui_demo_lib/src/demo/toggle_switch.rs#L109.
pub fn toggle(on: &mut bool) -> impl Widget + '_ {
	move |ui: &mut Ui| {
		let desired_size = ui.spacing().interact_size.y * vec2(2.0, 1.0);
		let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());

		if response.clicked() {
			*on = !*on;

			response.mark_changed();
		}
		if ui.is_rect_visible(rect) {
			let how_on = ui.ctx().animate_bool_responsive(response.id, *on);
			let visuals = ui.style().interact_selectable(&response, *on);
			let rect = rect.expand(visuals.expansion);
			let radius = 0.5 * rect.height();

			ui.painter().rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);

			let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
			let center = egui::pos2(circle_x, rect.center().y);

			ui.painter().circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
		}

		response
	}
}

pub fn image_button<S>(src: S, pixels: f32) -> Image<'static>
where
	S: Into<ImageSource<'static>>,
{
	Image::new(src).max_size((pixels, pixels).into()).sense(Sense::click())
}
