use eframe::egui::Ui;

pub enum Page {}

impl Page {
	pub fn update<Store: onlivfe::storage::OnlivfeStore>(
		&mut self, ui: &mut Ui, ctx: &eframe::egui::Context,
		i: &onlivfe_wrapper::Onlivfe<Store>,
	) {
		ui.heading("Add account");
	}
}
