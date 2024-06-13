pub use reqwew::{Http, Response};

// crates.io
use reqwew::{once_cell::sync::Lazy, Client};

pub static HTTP_CLIENT: Lazy<Client> = reqwew::lazy(Default::default);
