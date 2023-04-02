use std::sync::Arc;

use crate::{HistoryBehavior, UpdatablePage};

#[derive(Debug, Clone)]
pub struct Page;
impl<Store: onlivfe::storage::OnlivfeStore + 'static> From<Page> for crate::Page<Store> {
	fn from(value: Page) -> Self { Self::About(value) }
}

impl Default for Page {
	fn default() -> Self { Self }
}

impl<Store: onlivfe::storage::OnlivfeStore + 'static> UpdatablePage<Store> for Page {
	fn update(
		&mut self, ui: &mut eframe::egui::Ui, _ctx: &eframe::egui::Context,
		_i: Arc<onlivfe_wrapper::Onlivfe<Store>>,
	) -> Option<(crate::Page<Store>, HistoryBehavior)> {
		ui.heading("About");

		None
	}
}
