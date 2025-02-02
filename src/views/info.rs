use dioxus::prelude::*;
use dioxus_i18n::tid;

#[component]
pub fn Info() -> Element {
	rsx! {
		section {
			h1 { {concat!("Tantalos v", env!("CARGO_PKG_VERSION"))} }

			p { {tid!("placeholder-text")} }
			p { {tid!("placeholder-text")} }
			p { {tid!("placeholder-text")} }
			p { {tid!("placeholder-text")} }
			p { {tid!("placeholder-text")} }
		}
	}
}
