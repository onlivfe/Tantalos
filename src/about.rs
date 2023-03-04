use std::sync::Arc;

use eframe::egui::Ui;

use crate::{HistoryBehavior, UpdatablePage};

#[derive(Debug, Clone)]
pub struct Page;
impl From<Page> for crate::Page {
	fn from(value: Page) -> Self { Self::About(value) }
}

impl Default for Page {
	fn default() -> Self { Self }
}

impl UpdatablePage for Page {
	fn update<Store: onlivfe::storage::OnlivfeStore + 'static>(
		&mut self, ui: &mut eframe::egui::Ui, ctx: &eframe::egui::Context,
		i: Arc<onlivfe_wrapper::Onlivfe<Store>>,
	) -> Option<(crate::Page, HistoryBehavior)> {
		ui.heading("About");

		None
	}
}
