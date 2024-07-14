// std
use std::sync::atomic::Ordering;
// crates.io
use eframe::egui::*;
// self
use super::super::UiT;
use crate::{air::AiRContext, component::openai::Model, util, widget};

#[derive(Debug, Default)]
pub struct Chat {
	input: String,
	output: String,
	shortcut: ShortcutWidget,
}
impl UiT for Chat {
	fn draw(&mut self, ui: &mut Ui, ctx: &mut AiRContext) {
		let dark_mode = ui.visuals().dark_mode;
		let size = ui.available_size();
		let is_chatting = ctx.services.is_chatting();

		ScrollArea::vertical().id_source("Input").max_height((size.y - 50.) / 2.).show(ui, |ui| {
			let input = ui.add_sized(
				(size.x, ui.available_height()),
				TextEdit::multiline({
					if is_chatting {
						if let Some(i) = ctx.state.chat.input.try_read() {
							i.clone_into(&mut self.input);
						}
					}

					&mut self.input
				})
				.hint_text(&*ctx.state.chat.quote.read()),
			);

			if input.has_focus() {
				self.shortcut.copy.triggered = false;

				let to_send = input.ctx.input(|i| {
					let modifier = if cfg!(target_os = "macos") {
						i.modifiers.mac_cmd
					} else {
						i.modifiers.ctrl
					};

					modifier && i.key_pressed(Key::Enter)
				});

				if to_send {
					ctx.services.chat.send((
						ctx.components.setting.general.active_func.basic(),
						self.input.clone(),
						false,
					));
				}
			}
		});

		// Indicators.
		ui.horizontal(|ui| {
			ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
				ui.vertical(|ui| {
					ui.add_space(4.5);
					ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
						let tcs = &ctx.state.chat.token_counts;
						let (itc, otc) =
							(tcs.0.load(Ordering::Relaxed), tcs.1.load(Ordering::Relaxed));
						let (ip, op) = ctx.components.setting.ai.model.prices();

						ui.hyperlink_to(format!(
							"input: {itc} output: {otc} price: ${:.6}",
							util::price_rounded(itc as f32 * ip + otc as f32 * op)
						),Model::PRICE_URI).on_hover_text("The token indicator might not work if you are using a custom API provider.");
					});
				});
			});
		});

		ui.separator();

		// Shortcuts.
		ui.horizontal(|ui| {
			ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
				if is_chatting {
					ui.spinner();
				} else {
					// TODO: change retry to send.
					// TODO: the state will not be synced if previous action is triggered by hotkey.
					if ui.add(self.shortcut.retry.img(dark_mode)).clicked() {
						ctx.services.chat.send((
							ctx.components.setting.general.active_func.basic(),
							self.input.clone(),
							false,
						));
					}
				}
				if !self.shortcut.copy.triggered {
					if ui.add(self.shortcut.copy.copy_img(dark_mode)).clicked() {
						self.shortcut.copy.triggered = true;
						ctx.components
							.clipboard
							.set_text(&self.output)
							.expect("clipboard must be available");
					}
				} else {
					ui.add(self.shortcut.copy.copied_img(dark_mode));
				}
			});
		});

		ScrollArea::vertical().id_source("Output").show(ui, |ui| {
			ui.label({
				if is_chatting {
					if let Some(o) = ctx.state.chat.output.try_read() {
						o.clone_into(&mut self.output);
					}
				}

				&self.output
			});
		});
	}
}

#[derive(Debug, Default)]
struct ShortcutWidget {
	copy: CopyWidget,
	retry: RetryWidget,
}
#[derive(Debug)]
struct CopyWidget {
	copy_img_l: Image<'static>,
	copy_img_d: Image<'static>,
	copied_img_l: Image<'static>,
	copied_img_d: Image<'static>,
	triggered: bool,
}
impl CopyWidget {
	fn copy_img(&self, dark_mode: bool) -> Image<'static> {
		if dark_mode {
			self.copy_img_d.clone()
		} else {
			self.copy_img_l.clone()
		}
	}

	fn copied_img(&self, dark_mode: bool) -> Image<'static> {
		if dark_mode {
			self.copied_img_d.clone()
		} else {
			self.copied_img_l.clone()
		}
	}
}
impl Default for CopyWidget {
	fn default() -> Self {
		Self {
			copy_img_d: widget::image_button(include_image!("../../../asset/copy-dark.svg")),
			copy_img_l: widget::image_button(include_image!("../../../asset/copy-light.svg")),
			copied_img_d: widget::image_button(include_image!("../../../asset/copied-dark.svg")),
			copied_img_l: widget::image_button(include_image!("../../../asset/copied-light.svg")),
			triggered: false,
		}
	}
}
#[derive(Debug)]
struct RetryWidget {
	retry_img_d: Image<'static>,
	retry_img_l: Image<'static>,
}
impl RetryWidget {
	fn img(&self, dark_mode: bool) -> Image<'static> {
		if dark_mode {
			self.retry_img_d.clone()
		} else {
			self.retry_img_l.clone()
		}
	}
}
impl Default for RetryWidget {
	fn default() -> Self {
		Self {
			retry_img_d: widget::image_button(include_image!("../../../asset/retry-dark.svg")),
			retry_img_l: widget::image_button(include_image!("../../../asset/retry-light.svg")),
		}
	}
}
