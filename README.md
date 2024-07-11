<div align="center">

# AiR

<h3 style="display: flex; align-items: center; justify-content: center;">
	AI with Rust ｜ <img src="asset/icon.png" alt="App Icon" style="width: 105; height: 105; margin-left: 10px;">
</h3>

[![License](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Checks](https://github.com/hack-ink/air/actions/workflows/checks.yml/badge.svg?branch=main)](https://github.com/hack-ink/air/actions/workflows/checks.yml)
[![Release](https://github.com/hack-ink/air/actions/workflows/release.yml/badge.svg)](https://github.com/hack-ink/air/actions/workflows/release.yml)
[![GitHub tag (latest by date)](https://img.shields.io/github/v/tag/hack-ink/air)](https://github.com/hack-ink/air/tags)
[![GitHub last commit](https://img.shields.io/github/last-commit/hack-ink/air?color=red&style=plastic)](https://github.com/hack-ink/air)

</div>

## Architecture
Built upon [egui](https://github.com/emilk/egui), a fast and cross-platform GUI toolkit written in pure Rust.

### Components
- **Static**: These items are static and are used by other parts of the application.

### OS
- **Interaction**: Provides wrapped APIs to interact with the operating system.

### Services
- **Background Processes**: These items are time-sensitive and require frequent checking or updating. They will be spawned as separate threads and run in the background.

### State
- **Synchronization**: Mutable version of the components. Usually, they are `Arc<Mutex<Components>>` in order to sync the state between service and UI.

### UI
- **User Interface**: The user interface components.


## License
AiR is licensed under the [GPLv3](https://www.gnu.org/licenses/gpl-3.0) license.
