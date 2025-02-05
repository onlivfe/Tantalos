use dioxus::prelude::*;

use crate::tid;

#[component]
pub fn Info() -> Element {
	rsx! {
		section {
			h1 { {concat!("Tantalos v", env!("CARGO_PKG_VERSION"))} }

			p { {tid!("about-app")} }
			p { {tid!("placeholder-text")} }
			p { {tid!("placeholder-text")} }
			p { {tid!("placeholder-text")} }
			p { {tid!("placeholder-text")} }
		}
	}
}
