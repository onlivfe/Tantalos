use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::components::{LanguagePicker, LayoutPicker};

#[component]
pub fn Settings() -> Element {
	rsx! {
		section {
			h1 { { t!("settings") } }

				LanguagePicker {
					compact: false
				}
				LayoutPicker { }
		}
	}
}
