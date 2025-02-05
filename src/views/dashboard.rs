use dioxus::prelude::*;

use crate::tid;

#[component]
pub fn Dashboard() -> Element {
	rsx! {
		section {
			h1 { {tid!("dashboard")} }
		}
	}
}
