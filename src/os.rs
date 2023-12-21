mod macos;

pub(super) trait Accessibility {
	fn hide_application();

	fn activate_application();

	fn get_selected_text() -> Option<String>;
}

#[derive(Debug)]
pub(super) struct Os;
