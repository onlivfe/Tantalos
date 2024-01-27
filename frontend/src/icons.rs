use leptos::*;

#[component]
#[must_use]
pub fn icon(#[prop(into)] name: MaybeSignal<String>) -> impl IntoView {
	view! { <i class="icon">{name}</i> }
}
