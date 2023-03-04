use std::sync::Arc;

use eframe::egui::Ui;

use crate::{HistoryBehavior, UpdatablePage};

#[derive(Debug, Clone)]
pub enum Page {
	Loading,
}
impl From<Page> for crate::Page {
	fn from(value: Page) -> Self { Self::Dash(value) }
}

impl Default for Page {
	fn default() -> Self { Self::Loading }
}

impl UpdatablePage for Page {
	fn update<Store: onlivfe::storage::OnlivfeStore>(
		&mut self, ui: &mut Ui, ctx: &eframe::egui::Context,
		i: Arc<onlivfe_wrapper::Onlivfe<Store>>,
	) -> Option<(crate::Page, HistoryBehavior)> {
		ui.heading("Dash");

		None
	}
}
