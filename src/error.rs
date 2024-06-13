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
	Enigo(#[from] EnigoError),
	#[error(transparent)]
	GlobalHotKey(#[from] global_hotkey::Error),
	#[error(transparent)]
	OpenAi(#[from] async_openai::error::OpenAIError),
	#[error(transparent)]
	Toml(#[from] toml::de::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum EnigoError {
	#[error(transparent)]
	Input(#[from] enigo::InputError),
	#[error(transparent)]
	NewCon(#[from] enigo::NewConError),
}
