#[cfg(target_os = "macos")] mod macos;

pub trait Accessibility {
	fn hide();

	fn unhide();

	fn selected_text() -> Option<String>;
}

#[derive(Debug)]
pub struct Os;
