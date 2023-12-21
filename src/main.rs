// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod hotkey;
use hotkey::Function;

mod os;

mod preference;

mod widget;
use widget::*;

// std
use std::{
	sync::{mpsc::Receiver, Arc, Mutex},
	thread,
	time::Duration,
};
// crates.io
use anyhow::Result;
use clipboard::{ClipboardContext, ClipboardProvider};
use eframe::{
	egui::{
		CentralPanel, Context, FontData, FontDefinitions, FontFamily, ScrollArea, Slider, TextEdit,
		TextStyle, ViewportBuilder,
	},
	App, CreationContext, Frame, NativeOptions,
};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};

fn main() -> Result<()> {
	color_eyre::install().map_err(|e| anyhow::anyhow!(e))?;
	tracing_subscriber::fmt::init();

	eframe::run_native(
		"AIR",
		NativeOptions {
			run_and_return: true,
			viewport: ViewportBuilder::default().with_inner_size((480., 320.)),
			..Default::default()
		},
		Box::new(|cc| Box::new(Air::new(cc))),
	)
	.map_err(|e| anyhow::anyhow!("{e}"))?;

	Ok(())
}

// #[derive(Debug)]
struct Air {
	on_initialize: Arc<Mutex<bool>>,
	clipboard: Arc<Mutex<ClipboardContext>>,
	hotkey_rx: Receiver<Function>,
	api_key: ApiKey,
	temperature: f64,
	input: String,
	output: String,
}
impl Air {
	fn new(creation_ctx: &CreationContext) -> Self {
		let on_initialize = Arc::new(Mutex::new(true));

		{
			let ctx = creation_ctx.egui_ctx.clone();
			let on_init = on_initialize.clone();

			// Give some time to all components to initialize themselves.
			thread::spawn(move || {
				loop {
					if !ctx.input(|i| i.focused) {
						thread::sleep(Duration::from_millis(10));
					} else {
						break;
					}
				}

				*on_init.lock().unwrap() = false;
			});
		}

		let clipboard = Arc::new(Mutex::new(clipboard::ClipboardContext::new().unwrap()));

		{
			let fonts = {
				let mut f = FontDefinitions::default();

				f.font_data.insert(
					"Monaspace Radon Var".into(),
					FontData::from_static(include_bytes!(
						"../asset/MonaspaceRadonVarVF[wght,wdth,slnt].ttf"
					)),
				);
				f.families
					.entry(FontFamily::Proportional)
					.or_default()
					.insert(0, "Monaspace Radon Var".into());
				f.families
					.entry(FontFamily::Monospace)
					.or_default()
					.insert(0, "Monaspace Radon Var".into());

				f
			};

			creation_ctx.egui_ctx.set_fonts(fonts);
			creation_ctx.egui_ctx.style_mut(|s| {
				s.text_styles.get_mut(&TextStyle::Body).unwrap().size = 13.;
				s.text_styles.get_mut(&TextStyle::Monospace).unwrap().size = 13.;
			});
		}

		Self {
			on_initialize,
			clipboard: clipboard.clone(),
			hotkey_rx: hotkey::register(clipboard).unwrap(),
			api_key: Default::default(),
			temperature: 0.7,
			input: Default::default(),
			output: Default::default(),
		}
	}
}
impl App for Air {
	fn update(&mut self, ctx: &Context, _: &mut Frame) {
		CentralPanel::default().show(ctx, |ui| {
			if !*self.on_initialize.lock().unwrap() && !ctx.input(|i| i.focused) {
				os::hide_application();
			}

			let size = ui.available_size();

			ui.horizontal(|ui| {
				ui.horizontal(|ui| {
					ui.set_width(100.);
					ui.label("API Key");
				});
				ui.horizontal(|ui| {
					ui.add_sized(
						(280., ui.available_height()),
						TextEdit::singleline(&mut self.api_key.value)
							.password(self.api_key.password),
					);
				});

				if ui.button(&self.api_key.label).clicked() {
					self.api_key.clicked();
				}
			});
			ui.horizontal(|ui| {
				ui.horizontal(|ui| {
					ui.set_width(100.);
					ui.label("Temperature");
				});
				ui.horizontal(|ui| {
					ui.spacing_mut().slider_width = 280.;
					ui.add(
						Slider::new(&mut self.temperature, 0_f64..=1.)
							.step_by(0.01)
							.max_decimals(2),
					);
				});
			});

			if let Ok(hk) = self.hotkey_rx.try_recv() {
				match hk {
					Function::Polish => {
						self.input = self.clipboard.try_lock().map_or_else(
							|_| Default::default(),
							|mut c| c.get_contents().unwrap_or_default(),
						);
					},
				}
			}

			{
				ui.label("Input");

				ScrollArea::vertical().id_source("Input").max_height(100.).show(ui, |ui| {
					ui.add_sized(
						(size.x, ui.available_height()),
						TextEdit::multiline(&mut self.input),
					);
				});
			}
			{
				ui.label("Output");

				CommonMarkViewer::new("Output").show_scrollable(
					ui,
					&mut CommonMarkCache::default(),
					&self.input,
				);
			}
		});
	}
}
