use std::sync::Arc;

use eframe::{egui::Ui, epaint::mutex::Mutex};
use onlivfe::PlatformType;
use strum::IntoEnumIterator;

use crate::{HistoryBehavior, UpdatablePage};

pub struct Page {
	/// Non-UI thread needs to drop lock as quickly as possible
	data: Arc<Mutex<Vec<PlatformType>>>,
}
impl From<Page> for crate::Page {
	fn from(value: Page) -> Self { Self::Settings(value) }
}

impl Page {
	pub fn new<Store: onlivfe::storage::OnlivfeStore + 'static>(
		i: Arc<onlivfe_wrapper::Onlivfe<Store>>,
	) -> Self {
		let data: Arc<Mutex<Vec<PlatformType>>> = Arc::default();
		let page = Self { data: data.clone() };

		tokio::spawn(async move {
			for platform in PlatformType::iter() {
				if i.check_auth(platform).await.is_ok() {
					data.lock().push(platform);
				};
			}
		});

		page
	}
}

impl UpdatablePage for Page {
	fn update<Store: onlivfe::storage::OnlivfeStore + 'static>(
		&mut self, ui: &mut Ui, ctx: &eframe::egui::Context,
		i: Arc<onlivfe_wrapper::Onlivfe<Store>>,
	) -> Option<(crate::Page, HistoryBehavior)> {
		ui.heading("Settings");
		let mut any_missing_auth = false;
		for platform in PlatformType::iter() {
			if self.data.lock().contains(&platform) {
				ui.label("Authenticated on ".to_owned() + platform.as_ref());
			} else {
				any_missing_auth = true;
				ui.label("Not authenticated on ".to_owned() + platform.as_ref());
			}
		}
		if any_missing_auth && ui.button("Authenticate an account").clicked() {
			return Some((
				crate::add_account::Page::new(i).into(),
				HistoryBehavior::Skip,
			));
		}

		None
	}
}
