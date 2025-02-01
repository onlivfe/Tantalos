use std::fmt::{Display};

mod navbar;
pub use navbar::Navbar;

mod icon;
pub use icon::Icon;

mod layout;
pub use layout::{Layout, LayoutConfig, LayoutPicker};

mod lang;
pub use lang::{I18nConf, LanguagePicker, i18n_config};

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub enum VerticalDirection {
	Down,
	Up,
}

impl VerticalDirection {
	pub fn as_str(&self) -> &'static str {
		match self {
			Self::Down => "Down",
			Self::Up => "Up",
		}
	}
}

impl Display for VerticalDirection {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.as_str())
	}
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub enum HorizontalDirection {
	Left,
	Right,
}

impl HorizontalDirection {
	pub fn as_str(&self) -> &'static str {
		match self {
			Self::Left => "Left",
			Self::Right => "Right",
		}
	}
}

impl Display for HorizontalDirection {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.as_str())
	}
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub enum Direction {
	Vertical(VerticalDirection),
	Horizontal(HorizontalDirection),
}

impl Direction {
	pub fn as_str(&self) -> &'static str {
		match self {
			Self::Vertical(v) => v.as_str(),
			Self::Horizontal(h) => h.as_str(),
		}
	}
}

impl Display for Direction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.as_str())
	}
}
