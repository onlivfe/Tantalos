use std::sync::Arc;

use eframe::egui::Ui;

use crate::{HistoryBehavior, UpdatablePage};

#[derive(Debug, Clone)]
pub struct Page {}
impl From<Page> for crate::Page {
	fn from(value: Page) -> Self { Self::AddAccount(value) }
}

impl Page {
	pub fn new<Store: onlivfe::storage::OnlivfeStore + 'static>(
		i: Arc<onlivfe_wrapper::Onlivfe<Store>>,
	) -> Self {
		Self {}
	}
}

impl UpdatablePage for Page {
	fn update<Store: onlivfe::storage::OnlivfeStore>(
		&mut self, ui: &mut Ui, ctx: &eframe::egui::Context,
		i: Arc<onlivfe_wrapper::Onlivfe<Store>>,
	) -> Option<(crate::Page, HistoryBehavior)> {
		ui.heading("Add account");

		None
	}
}
