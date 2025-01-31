use dioxus::prelude::*;

#[component]
pub fn Icon(name: String) -> Element {
	rsx! {
		i {
			class: "icon",
			{ name }
		}
	}
}
