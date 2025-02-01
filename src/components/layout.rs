use dioxus::prelude::*;

use crate::{
	Route,
	components::{Footer, Navbar},
};

#[component]
pub fn Layout() -> Element {
	rsx! {
		aside {
			Navbar {  },
		}
		div {
			class: "content",
			main {
				class: "container",

					Outlet::<Route> {},
			}
			Footer { }
		}
	}
}
