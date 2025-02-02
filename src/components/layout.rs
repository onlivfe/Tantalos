use dioxus::prelude::*;
use dioxus_i18n::tid;

use super::{Direction, HorizontalDirection};
use crate::{
	Route,
	components::{Icon, Navbar, VerticalDirection},
};

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct LayoutConfig {
	nav_side: Direction,
}

impl Default for LayoutConfig {
	fn default() -> Self {
		Self { nav_side: Direction::Horizontal(HorizontalDirection::Left) }
	}
}

pub fn Layout() -> Element {
	let config_signal = use_context::<Signal<LayoutConfig>>();
	let config = config_signal.read();
	let sidenav = matches!(config.nav_side, Direction::Horizontal(_));
	let alt_dir = match config.nav_side {
		Direction::Horizontal(v) => match v {
			HorizontalDirection::Left => false,
			HorizontalDirection::Right => true,
		},
		Direction::Vertical(v) => match v {
			VerticalDirection::Up => false,
			VerticalDirection::Down => true,
		},
	};
	rsx! {
		if sidenav {
			aside { class: if alt_dir { "right" } else { "left" }, Navbar {} }
		} else {
			if alt_dir {
				footer { Navbar {} }
			} else {
				header { Navbar {} }
			}
		}
		main { class: "container-fluid", Outlet::<Route> {} }
	}
}

#[component]
pub fn LayoutPicker() -> Element {
	let mut config_signal = use_context::<Signal<LayoutConfig>>();
	let nav_side = config_signal.read().nav_side;
	let directions = vec![
		(Direction::Horizontal(HorizontalDirection::Left), "arrow_back"),
		(Direction::Horizontal(HorizontalDirection::Right), "arrow_forward"),
		(Direction::Vertical(VerticalDirection::Up), "arrow_upward"),
		(Direction::Vertical(VerticalDirection::Down), "arrow_downward"),
	];
	rsx! {
		details { class: "dropdown",
			summary { class: "outline", role: "button",
				Icon { name: "menu_open" }
				{" "}
				span { {tid!("menu-side", selector : "true", side : nav_side.as_str())} }
			}
			ul {
				for (direction , icon_name) in directions.clone() {
					li {
						a {
							aria_current: direction == nav_side,
							onclick: move |_| {
							    config_signal.write().nav_side = direction;
							},
							Icon { name: icon_name }
							{tid!("menu-side", selector : "false", side : direction.as_str())}
						}
					}
				}
			}
		}
	}
}
