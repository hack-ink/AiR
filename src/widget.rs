// crates.io
use eframe::egui::{self, *};

pub trait ComboBoxItem
where
	Self: Sized + Clone + PartialEq,
{
	const COUNT: usize;

	type Array: AsRef<[Self]>;

	// TODO: Rust doesn't support generic const yet.
	// `[Self; Self::COUNT]` is not allowed.
	fn all() -> Self::Array;

	fn as_str(&self) -> &'static str;
}

pub fn combo_box<'a, I>(label: &'a str, current: &'a mut I) -> impl Widget + 'a
where
	I: Clone + PartialEq + ComboBoxItem,
{
	move |ui: &mut Ui| {
		ui.label(label);

		let mut resp =
			ComboBox::from_id_source(label).selected_text(current.as_str()).show_ui(ui, |ui| {
				I::all().as_ref().iter().fold(false, |changed, i| {
					changed | ui.selectable_value(current, i.to_owned(), i.as_str()).changed()
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
