<div align="center">

# AiR
### AI with Rust.

[![License](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Checks](https://github.com/hack-ink/air/actions/workflows/checks.yml/badge.svg?branch=main)](https://github.com/hack-ink/air/actions/workflows/checks.yml)
[![Release](https://github.com/hack-ink/air/actions/workflows/release.yml/badge.svg)](https://github.com/hack-ink/air/actions/workflows/release.yml)
[![GitHub tag (latest by date)](https://img.shields.io/github/v/tag/hack-ink/air)](https://github.com/hack-ink/air/tags)
[![GitHub code lines](https://tokei.rs/b1/github/hack-ink/air)](https://github.com/hack-ink/air)
[![GitHub last commit](https://img.shields.io/github/last-commit/hack-ink/air?color=red&style=plastic)](https://github.com/hack-ink/air)

</div>

## Architecture
Built upon [egui](https://github.com/emilk/egui), a fast and cross-platform GUI toolkit written in pure Rust.

### Components
These items either have their own `refresh` logic or do not require frequent refreshing.
They are not time-sensitive, and their `refresh` method will be called at specific intervals (e.g., every 15 seconds).

### OS
Provides wrapped APIs to interact with the operating system.

### Services
These items are time-sensitive and require frequent checking or updating.
They will be spawned as separate threads and run in the background.

### UI
The user interface components.
