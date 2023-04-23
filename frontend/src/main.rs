//! Rust powered [onlivfe](https://onlivfe.com)'s VR Peeps frontend.
//!
//! Very WIP alternative to compete against the [angular based desktop app](https://github.com/onlivfe/desktop).

#![cfg_attr(nightly, feature(doc_auto_cfg))]
#![deny(clippy::all)]
#![forbid(unsafe_code)]
#![deny(clippy::cargo)]
#![deny(rustdoc::invalid_html_tags)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
// My project my choice, tabs are literally made for indentation, spaces not.
#![allow(clippy::tabs_in_doc_comments)]
// Not much can be done about it :/
#![allow(clippy::multiple_crate_versions)]

use instances::Instances;
use peeps::Peeps;
use tauri_sys::tauri::invoke;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

mod icons;
mod instances;
mod peeps;
mod settings;
pub use icons::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
	#[at("/peeps")]
	Peeps,
	#[at("/instances")]
	Instances,
	#[at("/settings")]
	SettingsRoot,
	#[at("/settings/*")]
	Settings,
	#[at("/save-and-exit")]
	SaveAndExit,
	#[not_found]
	#[at("/404")]
	NotFound,
}

#[allow(clippy::needless_pass_by_value)]
fn switch_route(route: Route) -> Html {
	match route {
		Route::Peeps => html! {<Peeps/>},
		Route::Instances => html! {<Instances/>},
		Route::SettingsRoot | Route::Settings => {
			html! { <Switch<settings::Route> render={settings::switch_route} /> }
		}
		Route::NotFound => {
			html! {<Redirect<Route> to={Route::Peeps}/>}
		}
		Route::SaveAndExit => html! {<SaveAndExit/>},
	}
}

#[function_component(SaveAndExit)]
fn save_and_exit() -> Html {
	let closing = use_async_with_options(
		async move {
			match invoke::<_, ()>("save_and_exit", &()).await {
				Ok(_) => Ok(()),
				Err(e) => Err(e.to_string()),
			}
		},
		UseAsyncOptions::enable_auto(),
	);

	if closing.loading {
		html! {
			<>
				<h1>{"Saving..."}</h1>
				<progress></progress>
			</>
		}
	} else if let Some(error) = closing.error.as_ref() {
		html! {
			<section>
				<h1>{"Error exiting"}</h1>
				<details>
					<summary>{"Details"}</summary>
					<code>{error}</code>
				</details>
			</section>
		}
	} else {
		html! {
			<>
				<h1>{"Window closed"}</h1>
				<p>{"The window closing code ran successfully, so if you're reading this, something went very wrong."}</p>
			</>
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// How the page should get added to history
pub enum HistoryBehavior {
	/// Skips adding the page to history
	Skip,
	/// Add the page to history normally
	Add,
	/// Clear all of the history before the page
	Overwrite,
}

#[derive(Properties, PartialEq)]
pub struct TwoWayBindingProps<T: PartialEq> {
	pub value: T,
	pub onchange: Callback<T>,
}


#[function_component(App)]
fn app() -> Html {
	struct NavLink {
		pub to: Route,
		pub icon: &'static str,
		pub label: &'static str,
	}

	let nav_submenus = vec![
		vec![
			NavLink { to: Route::Peeps, icon: "group", label: "Friends" },
			NavLink {
				to: Route::Instances,
				icon: "travel_explore",
				label: "Instances",
			},
		],
		vec![
			NavLink { to: Route::SettingsRoot, icon: "settings", label: "Settings" },
			NavLink {
				to: Route::SaveAndExit,
				icon: "close",
				label: "Close the window",
			},
		],
	]
	.into_iter()
	.map(|subnav| {
		html! {
			<li>
				<ul>
				{
				subnav
					.into_iter()
					.map(|nav_link| {
						html! {
							<li title={nav_link.label}><Link<Route> to={nav_link.to}><Icon name={nav_link.icon}/></Link<Route>></li>
						}})
					.collect::<Html>()
				}
				</ul>
			</li>
		}
})
	.collect::<Html>();

	html! {
		<BrowserRouter>
			<nav>
				<ul>
					{nav_submenus}
				</ul>
			</nav>
			<main>
				<Switch<Route> render={switch_route} />
			</main>
		</BrowserRouter>
	}
}

fn main() {
	// Router breaks if the following is enabled
	//yew::set_event_bubbling(false);
	yew::Renderer::<App>::new().render();
}
