#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error(transparent)]
	Io(#[from] std::io::Error),
	#[error(transparent)]
	TryRecv(#[from] std::sync::mpsc::TryRecvError),

	#[error(transparent)]
	AppDirs2(#[from] app_dirs2::AppDirsError),
	#[error(transparent)]
	Eframe(#[from] eframe::Error),
	#[error(transparent)]
	GlobalHotKey(#[from] global_hotkey::Error),
	#[error(transparent)]
	Toml(#[from] toml::de::Error),
}
