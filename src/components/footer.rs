use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::components::LanguagePicker;

#[component]
pub fn Footer() -> Element {
	rsx! {
		footer {
			LanguagePicker {}
			p { {t!("tantalos") + concat!(" v", env!("CARGO_PKG_VERSION"))} }
		}
	}
}
