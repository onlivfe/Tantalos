use std::sync::Arc;

use onlivfe::storage::OnlivfeStore;

use crate::{HistoryBehavior, UpdatablePage};

/// The onlivfe app
pub struct Onlivfe<Store: OnlivfeStore> {
	/// The Onlvife interface
	i: Arc<onlivfe_wrapper::Onlivfe<Store>>,
	page: crate::Page,
	history: Vec<crate::Page>,
}

impl<Store: OnlivfeStore> Onlivfe<Store> {
	/// Creates a new onlivfe app
	pub fn new(
		creation_ctx: &eframe::CreationContext<'_>,
		interface: onlivfe_wrapper::Onlivfe<Store>,
	) -> Self {
		crate::fonts::setup(&creation_ctx.egui_ctx);

		let app = Self {
			i: Arc::new(interface),
			page: crate::Page::Dash(crate::dash::Page::default()),
			history: vec![],
		};

		let waker_ctx = creation_ctx.egui_ctx.clone();
		tokio::spawn(async move {
			// TODO: Setup background listeners
			waker_ctx.request_repaint();
		});

		app
	}
}

impl<Store: OnlivfeStore + 'static> eframe::App for Onlivfe<Store> {
	fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
		// On web, the browser controls the gui zoom.
		if !frame.is_web() {
			eframe::egui::gui_zoom::zoom_with_keyboard_shortcuts(
				ctx,
				frame.info().native_pixels_per_point,
			);
		}

		eframe::egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
			eframe::egui::menu::bar(ui, |ui| {
				if ui.button("Home").clicked() {
					self.page = crate::Page::Dash(crate::dash::Page::default());
				}
				ui.separator();
				if ui.button("About").clicked() {
					self.page = crate::Page::About(crate::about::Page::default());
				}
				ui.separator();
				if ui.button("Settings").clicked() {
					self.page =
						crate::Page::Settings(crate::settings::Page::new(self.i.clone()));
				}
				ui.separator();
				if ui.button("Quit").clicked() {
					frame.close();
				}
			});
		});

		eframe::egui::CentralPanel::default().show(ctx, |ui| {
			eframe::egui::ScrollArea::vertical().show(ui, |ui| {
				let next_page = match &mut self.page {
					crate::Page::About(page) => page.update(ui, ctx, self.i.clone()),
					crate::Page::AddAccount(page) => page.update(ui, ctx, self.i.clone()),
					crate::Page::Dash(page) => page.update(ui, ctx, self.i.clone()),
					crate::Page::Settings(page) => page.update(ui, ctx, self.i.clone()),
				};

				if let Some((mut page, overwrite_history)) = next_page {
					std::mem::swap(&mut self.page, &mut page);
					if overwrite_history == HistoryBehavior::Add {
						self.history.push(page);
					} else if overwrite_history == HistoryBehavior::Overwrite {
						self.history.clear();
					}
				}
			});
		});
	}
}
