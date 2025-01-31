use dioxus::prelude::*;

const HEADER_SVG: Asset = asset!("/res/icons/icon.svg");

#[component]
pub fn About() -> Element {
	rsx! {
			div {
					id: "hero",
					img { src: HEADER_SVG, id: "header" }
					div { id: "links",
							a { href: "https://onlivfe.com", "Learn about Onlivfe" }
					}
			}
	}
}
