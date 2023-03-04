use onlivfe::storage::OnlivfeStore;

/// The onlivfe app
pub struct Onlivfe<Store: OnlivfeStore> {
	/// The Onlvife interface
	i: onlivfe_wrapper::Onlivfe<Store>,
	page: Page,
}

pub enum Page {
	About(crate::about::Page),
	Settings(crate::settings::Page),
	AddAccount(crate::add_account::Page),
	Dash(crate::dash::Page),
}

impl<Store: OnlivfeStore> Onlivfe<Store> {
	/// Creates a new onlivfe app
	pub fn new(
		creation_ctx: &eframe::CreationContext<'_>,
		interface: onlivfe_wrapper::Onlivfe<Store>,
	) -> Self {
		crate::fonts::setup(&creation_ctx.egui_ctx);

		let app =
			Self { i: interface, page: Page::Dash(crate::dash::Page::default()) };

		let waker_ctx = creation_ctx.egui_ctx.clone();
		tokio::spawn(async move {
			// TODO: Setup background listeners
			waker_ctx.request_repaint();
		});

		app
	}
}

impl<Store: OnlivfeStore> eframe::App for Onlivfe<Store> {
	fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
		eframe::egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
			eframe::egui::menu::bar(ui, |ui| {
				if ui.button("Home").clicked() {
					self.page = Page::Dash(crate::dash::Page::default());
				}
				ui.separator();
				if ui.button("About").clicked() {
					self.page = Page::About(crate::about::Page::default());
				}
				ui.separator();
				if ui.button("Settings").clicked() {
					self.page = Page::Settings(crate::settings::Page::default());
				}
				ui.separator();
				if ui.button("Quit").clicked() {
					frame.close();
				}
			});
		});

		eframe::egui::CentralPanel::default().show(ctx, |ui| {
			eframe::egui::ScrollArea::vertical().show(ui, |ui| {
				match &mut self.page {
					Page::About(page) => page.update(ui, ctx, &self.i),
					Page::AddAccount(page) => page.update(ui, ctx, &self.i),
					Page::Dash(page) => page.update(ui, ctx, &self.i),
					Page::Settings(page) => page.update(ui, ctx, &self.i),
				}
			});
		});
	}
}
