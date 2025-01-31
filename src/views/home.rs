use dioxus::prelude::*;

use crate::components::About;

#[component]
pub fn Home() -> Element {
	rsx! {
		section {
			h1 { "Onlivfe Tantalos" }

			About {}
		}
	}
}
