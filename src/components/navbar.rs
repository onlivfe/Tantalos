use dioxus::prelude::*;
use dioxus_i18n::tid;
use dioxus_router::components::Link;

use crate::{Route, components::Icon};

const HEADER_SVG: Asset = asset!("/res/icons/icon.svg");

#[component]
pub fn Navbar() -> Element {
	let routes = vec![
		(tid!("accounts"), "manage_accounts", Route::Accounts {}),
		(tid!("settings"), "settings", Route::Settings {}),
		(tid!("info"), "info", Route::Info {}),
	];

	rsx! {
		nav {
			ul {
				li {
					ul {
						for (route_name , icon , route) in routes {
							li {
								Link {
									active_class: "active",
									to: route,
									role: "button",
									title: route_name,
									Icon { name: icon }
								}
							}
						}
					}
				}
				li {
					a {
						role: "button",
						class: "outline secondary",
						rel: "external",
						href: "https://onlivfe.com",
						title: tid!("to-project-url"),
						img { src: HEADER_SVG, id: "header" }
					}
				}
			}
		}
	}
}
