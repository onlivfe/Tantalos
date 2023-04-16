use std::sync::Arc;

use onlivfe::storage::OnlivfeStore;

use crate::{HistoryBehavior, UpdatablePage};

/// Could also be called "window"
struct View<Store: onlivfe::storage::OnlivfeStore + 'static> {
	page: crate::Page<Store>,
	history: Vec<crate::Page<Store>>,
}

/// The onlivfe app
pub struct Onlivfe<Store: OnlivfeStore + 'static> {
	/// The Onlvife interface
	i: Arc<onlivfe_wrapper::Onlivfe<Store>>,
	/// The views, currently always only a single one.
	///
	/// Stored as a vec for possible multi window support in the future.
	views: Vec<View<Store>>,
}

impl<Store: OnlivfeStore> Onlivfe<Store> {
	/// Creates a new onlivfe app
	#[must_use]
	pub fn new(
		creation_ctx: &eframe::CreationContext<'_>,
		interface: onlivfe_wrapper::Onlivfe<Store>,
	) -> Self {
		crate::fonts::setup(&creation_ctx.egui_ctx);

		let view = View {
			page: crate::Page::Dash(crate::dash::Page::default()),
			history: vec![],
		};

		let app = Self { i: Arc::new(interface), views: vec![view] };

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

		for view in &mut self.views {
			eframe::egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
				eframe::egui::menu::bar(ui, |ui| {
					if ui.button("Home").clicked() {
						let mut page = crate::Page::Dash(crate::dash::Page::default());
						std::mem::swap(&mut view.page, &mut page);
						view.history.push(page);
					}
					ui.separator();
					if ui.button("About").clicked() {
						let mut page = crate::Page::About(crate::about::Page::default());
						std::mem::swap(&mut view.page, &mut page);
						view.history.push(page);
					}
					ui.separator();
					if ui.button("Settings").clicked() {
						let mut page = crate::Page::Settings(crate::settings::Page::new(self.i.clone()));
						std::mem::swap(&mut view.page, &mut page);
						view.history.push(page);
					}
					ui.separator();
					if ui.button("Quit").clicked() {
						frame.close();
					}
				});
			});

			eframe::egui::CentralPanel::default().show(ctx, |ui| {
				eframe::egui::ScrollArea::vertical().show(ui, |ui| {
						let next_page = view.page.update(ui, ctx, self.i.clone());

						if let Some((mut page, overwrite_history)) = next_page {
							std::mem::swap(&mut view.page, &mut page);
							if overwrite_history == HistoryBehavior::Add {
								view.history.push(page);
							} else if overwrite_history == HistoryBehavior::Overwrite {
								view.history.clear();
							}
						}
					
				});
			});
		}
	}
}
