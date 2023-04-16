use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct IconProps {
	pub name: String,
}

#[function_component(Icon)]
pub fn icon(props: &IconProps) -> Html {
	html! {
		<i class="icon">{&props.name}</i>
	}
}
