use std::sync::Arc;

use eframe::{egui::Ui, epaint::mutex::Mutex};
use onlivfe::PlatformType;
use strum::IntoEnumIterator;

use crate::{HistoryBehavior, UpdatablePage};

pub struct Page {
	/// Non-UI thread needs to drop lock as quickly as possible
	data: Arc<Mutex<Vec<PlatformType>>>,
}
impl<Store: onlivfe::storage::OnlivfeStore + 'static> From<Page> for crate::Page<Store> {
	fn from(value: Page) -> Self { Self::Settings(value) }
}

impl Page {
	#[must_use]
	pub fn new<Store: onlivfe::storage::OnlivfeStore + 'static>(
		i: Arc<onlivfe_wrapper::Onlivfe<Store>>,
	) -> Self {
		let data: Arc<Mutex<Vec<PlatformType>>> = Arc::default();
		let page = Self { data: data.clone() };

		tokio::spawn(async move {
			// TODO: Proper parallel
			for platform in PlatformType::iter() {
				if i.check_auth(platform).await.is_ok() {
					data.lock().push(platform);
				};
			}
		});

		page
	}
}

impl<Store: onlivfe::storage::OnlivfeStore + 'static> UpdatablePage<Store>
	for Page
{
	fn update(
		&mut self, ui: &mut Ui, _ctx: &eframe::egui::Context,
		i: Arc<onlivfe_wrapper::Onlivfe<Store>>,
	) -> Option<(crate::Page<Store>, HistoryBehavior)> {
		ui.heading("Settings");
		let mut any_missing_auth = false;
		{
			let data = self.data.lock();
			for platform in PlatformType::iter() {
				if data.contains(&platform) {
					ui.label("Authenticated on ".to_owned() + platform.as_ref());
				} else {
					any_missing_auth = true;
					ui.label("Not authenticated on ".to_owned() + platform.as_ref());
				}
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
