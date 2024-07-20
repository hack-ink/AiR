#[cfg(feature = "dev")]
fn main() {}

#[cfg(not(feature = "dev"))]
fn main() {
	#[cfg(target_os = "windows")]
	{
		// crates.io
		use winresource::WindowsResource;

		let mut res = WindowsResource::new();

		res.set_icon("asset/icon.ico");
		res.compile().unwrap();
	}
}
