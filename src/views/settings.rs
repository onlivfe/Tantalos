use dioxus::prelude::*;
use dioxus_i18n::tid;

use crate::components::{LanguagePicker, LayoutPicker};

#[component]
pub fn Settings() -> Element {
	rsx! {
		section {
			h1 { {tid!("settings")} }

			LanguagePicker { compact: false }
			LayoutPicker {}
		}
	}
}
