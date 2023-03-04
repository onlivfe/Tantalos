use eframe::egui::Ui;

pub struct Page;

impl Default for Page {
	fn default() -> Self { Self }
}

impl Page {
	pub fn update<Store: onlivfe::storage::OnlivfeStore>(
		&mut self, ui: &mut Ui, ctx: &eframe::egui::Context,
		i: &onlivfe_wrapper::Onlivfe<Store>,
	) {
		ui.heading("About");
	}
}
