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

use onlivfe::PlatformAccount;
use tauri_sys::tauri::invoke;
use yew::prelude::*;
use yew_hooks::prelude::*;

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

#[function_component(App)]
fn app() -> Html {
	let authenticated_accounts = use_async_with_options(
		async move {
			match invoke::<_, Vec<PlatformAccount>>("authenticated_accounts", &())
				.await
			{
				Ok(p) => Ok(p),
				Err(e) => Err(format!("Error: {:?}", e)),
			}
		},
		UseAsyncOptions::enable_auto(),
	);

	let on_login = Callback::from(move |ev: MouseEvent| {
		// Prevent actual form submission
		ev.prevent_default();
	});

	let add_account = html! {
		<>
			<form>
				<select>
					<option value="VRChat">{"VRChat"}</option>
					<option value="ChilloutVR">{"ChilloutVR"}</option>
					<option value="NeosVR">{"NeosVR"}</option>
				</select>
				<input type="text" placeholder="Login"/>
				<input type="password" placeholder="Password" />
				<button type="submit" onclick={on_login}>{"Login"}</button>
			</form>
		</>
	};

	if authenticated_accounts.loading {
		html! { "Loading" }
	} else if let Some(error) = authenticated_accounts.error.as_ref() {
		html! {
			<>
				<h1>{"Error loading accounts"}</h1>
				<p>{error}</p>
				{add_account}
			</>
		}
	} else {
		html! {
			<>
				<h1>{ "Accounts" }</h1>
				<ul>
						{ authenticated_accounts.data.as_ref().map_or_else(
							|| html!{},
							|v| v.iter().map(|acc| html!{ <li>{acc.id().id_as_string()}</li> }).collect::<Html>()
						)}
				</ul>
				{add_account}
			</>
		}
	}
}

fn main() {
	yew::set_event_bubbling(false);
	yew::Renderer::<App>::new().render();
}
