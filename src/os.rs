#[cfg(target_os = "macos")] mod macos;

pub trait Accessibility {
	fn get_selected_text() -> Option<String> {
		get_selected_text::get_selected_text().ok()
	}
}

pub trait AppKit {
	// #[deprecated(note = "use `ViewportCommand` instead")]
	fn hide();

	// #[deprecated(note = "use `ViewportCommand` instead")]
	fn unhide();

	fn set_move_to_active_space();
}

#[derive(Debug)]
pub struct Os;
