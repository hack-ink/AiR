// crates.io
use eframe::egui::*;
// self
use super::super::UiT;
use crate::air::AiRContext;
#[cfg(feature = "tokenizer")] use crate::component::util;

#[derive(Debug, Default)]
pub struct Chat {
	pub input: String,
	pub output: String,
	pub shortcut: ShortcutWidget,
}
impl UiT for Chat {
	fn draw(&mut self, ui: &mut Ui, ctx: &mut AiRContext) {
		let is_chatting = ctx.services.is_chatting();
		let size = ui.available_size();

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
		#[cfg(feature = "tokenizer")]
		ui.horizontal(|ui| {
			ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
				ui.vertical(|ui| {
					// TODO: maybe don't need this.
					// ui.add_space(4.5);
					ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
						let (ic, oc) =
							ctx.components.tokenizer.count_token(&self.input, &self.output);
						let (ip, op) = ctx.components.setting.ai.model.prices();

						ui.label(format!(
							"{} tokens (${:.6})",
							ic + oc,
							util::price_rounded(ic as f32 * ip + oc as f32 * op)
						));
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
					if ui.add(self.shortcut.retry.clone()).clicked() {
						ctx.services.chat.send((
							ctx.components.setting.general.active_func.basic(),
							self.input.clone(),
							false,
						));
					}
				}
				if !self.shortcut.copy.triggered {
					if ui.add(self.shortcut.copy.copy_img.clone()).clicked() {
						self.shortcut.copy.triggered = true;
						ctx.components
							.clipboard
							.set_text(&self.output)
							.expect("clipboard must be available");
					}
				} else {
					ui.add(self.shortcut.copy.copied_img.clone());
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

#[derive(Debug)]
pub struct ShortcutWidget {
	retry: Image<'static>,
	copy: CopyWidget,
}
impl Default for ShortcutWidget {
	fn default() -> Self {
		Self {
			retry: Image::new(include_image!("../../../asset/retry.svg"))
				.max_size((16., 16.).into())
				.sense(Sense::click()),
			copy: Default::default(),
		}
	}
}
// TODO: https://github.com/emilk/egui/issues/3453.
#[derive(Debug)]
pub struct CopyWidget {
	copy_img: Image<'static>,
	copied_img: Image<'static>,
	triggered: bool,
}
impl Default for CopyWidget {
	fn default() -> Self {
		Self {
			copy_img: Image::new(include_image!("../../../asset/copy.svg"))
				.max_size((16., 16.).into())
				.sense(Sense::click()),
			copied_img: Image::new(include_image!("../../../asset/check.svg"))
				.max_size((16., 16.).into()),
			triggered: false,
		}
	}
}
