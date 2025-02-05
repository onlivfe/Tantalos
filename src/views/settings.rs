use dioxus::prelude::*;

use crate::components::{LanguagePicker, LayoutPicker};
use crate::tid;

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
