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
// Leptos sadly exports from root without a prelude
#![allow(clippy::wildcard_imports)]
#![allow(clippy::future_not_send)]

use leptos::*;
use leptos_router::*;
use tauri_sys::tauri::invoke;

mod icons;
pub use icons::*;

mod routes;
pub use routes::*;

mod instances;
use instances::*;

mod peeps;
use peeps::*;

mod settings;
use settings::*;

#[component]
fn save_and_exit() -> impl IntoView {
	let closing = create_resource(
		|| (),
		|()| async move {
			match invoke::<_, ()>("save_and_exit", &()).await {
				Ok(()) => Ok(()),
				Err(e) => Err(e.to_string()),
			}
		},
	);

	view! {
		<h1>{"Saving..."}</h1>
		{move || match closing.get() {
			None => view! { <progress></progress> }.into_view(),
			Some(Err(error)) => {
				view! {
					<section>
						<h2>{"Error exiting"}</h2>
						<details>
							<summary>{"Details"}</summary>
							<code>{error}</code>
						</details>
					</section>
				}
					.into_view()
			}
			Some(Ok(())) => {
				view! {
					<h1>{"Window closed"}</h1>
					<p>
						{"The window closing code ran successfully, so if you're reading this, something went very wrong."}
					</p>
				}
					.into_view()
			}
		}}
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

#[component]
fn app() -> impl IntoView {
	struct NavLink {
		pub page: Page,
		pub icon: &'static str,
		pub label: &'static str,
	}

	let nav_links = vec![
		vec![
			NavLink { page: Page::Peeps, icon: "group", label: "Friends" },
			NavLink {
				page: Page::Instances,
				icon: "travel_explore",
				label: "Instances",
			},
		],
		vec![
			NavLink {
				page: Page::Settings(SettingsPage::Settings),
				icon: "settings",
				label: "Settings",
			},
			NavLink { page: Page::Quit, icon: "close", label: "Close the window" },
		],
	];

	view! {
		<Router>
			<nav>
				<ul>

					{nav_links
						.into_iter()
						.map(|subnav| {
							view! {
								<li>
									<ul>

										{subnav
											.into_iter()
											.map(|nav_link| {
												view! {
													<li title=nav_link.label>
														<A href=nav_link.page.path().into_owned()>
															<Icon name=nav_link.icon/>
														</A>
													</li>
												}
											})
											.collect::<Vec<_>>()}

									</ul>
								</li>
							}
						})
						.collect::<Vec<_>>()}

				</ul>
			</nav>
			<main>
				<Routes>
					<Route path=Page::Peeps.path() view=|| view! { <Peeps/> }/>
					<Route path=Page::Instances.path() view=|| view! { <Instances/> }/>
					<Route path="/settings" view=|| view! { <Outlet/> }>
						<Route path=SettingsPage::AddUser.path() view=|| view! { <AddAccount/> }/>
						<Route path=SettingsPage::Settings.path() view=|| view! { <Settings/> }/>
					</Route>
					<Route path=Page::Quit.path() view=|| view! { <SaveAndExit/> }/>
				</Routes>
			</main>
		</Router>
	}
}

fn main() {
	_ = console_log::init_with_level(log::Level::Debug);
	console_error_panic_hook::set_once();
	mount_to_body(|| {
		view! { <App/> }
	});
}
