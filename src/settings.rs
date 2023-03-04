use eframe::egui::Ui;

pub enum Page {
	Loading,
}

impl Default for Page {
	fn default() -> Self { Self::Loading }
}

impl Page {
	pub fn update<Store: onlivfe::storage::OnlivfeStore>(
		&mut self, ui: &mut Ui, ctx: &eframe::egui::Context,
		i: &onlivfe_wrapper::Onlivfe<Store>,
	) {
		ui.heading("Settings");
	}
}
