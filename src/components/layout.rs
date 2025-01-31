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
			Outlet::<Route> {},
			Footer {  }
		}
	}
}
