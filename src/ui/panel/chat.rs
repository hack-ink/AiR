// crates.io
use eframe::egui::*;
// self
use crate::{air::AiRContext, widget::SMALL_FONT_OFFSET};

#[cfg_attr(not(feature = "dev"), derive(Default))]
#[derive(Debug)]
pub struct Chat {
	pub input: String,
	pub output: String,
}
impl Chat {
	pub fn draw(&mut self, ctx: &mut AiRContext, ui: &mut Ui, bar_y: f32) {
		let is_chatting = ctx.services.is_chatting();

		if is_chatting {
			ctx.state.chat.input.try_sync_to(&mut self.input);
			ctx.state.chat.output.try_sync_to(&mut self.output);
		}

		let size = ui.min_rect().size();
		let h = size.y - bar_y * 2.;
		let separator_y = ui.spacing().item_spacing.y * 2.;
		// TODO: this isn't really the height.
		let shortcut_y = ctx.components.setting.general.font_size;
		let scroll_y = (h - separator_y - shortcut_y) / 2.;
		// let scroll_y = (h - shortcut_y) / 2.;

		// dbg!(size.y, h, shortcut_y, scroll_y);

		let dark_mode = ui.visuals().dark_mode;

		// Input.
		ui.vertical(|ui| {
			ui.set_height(scroll_y);

			ScrollArea::vertical().id_source("Input").show(ui, |ui| {
				let input = ui.add_sized(
					(size.x, scroll_y),
					TextEdit::multiline(&mut self.input).hint_text(&*ctx.state.chat.quote.read()),
				);

				if input.has_focus() {
					let to_send = input.ctx.input(|i| {
						#[cfg(target_os = "macos")]
						let modifier = i.modifiers.mac_cmd;
						#[cfg(not(target_os = "macos"))]
						let modifier = i.modifiers.ctrl;

						modifier && i.key_pressed(Key::Enter)
					});

					if to_send {
						ctx.services.chat.send((
							ctx.components.setting.chat.activated_function.basic(),
							self.input.clone(),
							false,
						));
					}
				}
			});
		});
		ui.separator();
		// Information.
		ui.horizontal(|ui| {
			let tip = if ctx.state.chat.error.load() {
				RichText::new(format!(
					"An error occurred while connecting with \"{}\". Press CTRL/META+ENTER to retry.",
					ctx.components.setting.ai.api_base
				))
				.color(Color32::RED)
			} else {
				RichText::new(if is_chatting {
					"Thinking..."
				} else {
					"Press CTRL/META+ENTER to send."
				})
				.color(if dark_mode { Color32::GOLD } else { Color32::BROWN })
			};

			ui.set_height(shortcut_y);
			ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
				ui.label(tip.size(ctx.components.setting.general.font_size - SMALL_FONT_OFFSET));
			});
		});
		ui.separator();
		// Output.
		ui.vertical(|ui| {
			ui.set_height(scroll_y);

			ScrollArea::vertical().id_source("Output").show(ui, |ui|
				// Read-only trick.
				ui.add_sized((size.x, scroll_y), TextEdit::multiline(&mut self.output.as_str())));
		});
	}
}
#[cfg(feature = "dev")]
impl Default for Chat {
	fn default() -> Self {
		const TEXT: &str = include_str!("../../../dev/text.txt");

		Self { input: TEXT.into(), output: TEXT.into() }
	}
}
