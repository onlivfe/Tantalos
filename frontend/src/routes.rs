use std::borrow::Cow;

#[derive(Debug, Clone, Copy, Default)]
pub enum Page {
	#[default]
	Peeps,
	Instances,
	Settings(SettingsPage),
	Quit,
}

impl Page {
	#[must_use]
	pub fn path(&self) -> Cow<str> {
		match self {
			Self::Peeps => "/peeps".into(),
			Self::Instances => "/instance".into(),
			Self::Settings(sub_page) => {
				format!("/settings/{}", sub_page.path()).into()
			}
			Self::Quit => "/save-and-exit".into(),
		}
	}
}

#[derive(Debug, Clone, Copy, Default)]
pub enum SettingsPage {
	#[default]
	Settings,
	AddUser,
}

impl SettingsPage {
	#[must_use]
	pub const fn path(&self) -> &'static str {
		match self {
			Self::Settings => "",
			Self::AddUser => "add-user",
		}
	}
}
