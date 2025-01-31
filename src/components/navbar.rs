use dioxus::prelude::*;
use dioxus_i18n::t;
use dioxus_router::components::Link;

use crate::{Route, components::Icon};

#[component]
pub fn Navbar() -> Element {
	let routes = vec![
		(t!("home"), "home", Route::Home {}),
		(t!("accounts"), "manage_accounts", Route::Accounts {}),
	];

	rsx! {
		ul {
			for (route_name, icon, route) in routes {
				li {
					Link {
						active_class: "active",
						to: route,
						role: "button",
						title: route_name,
						Icon {
							name: icon
						},
					}
				}
			}
		}
	}
}
