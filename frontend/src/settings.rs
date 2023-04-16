use onlivfe::PlatformAccount;
use tauri_sys::tauri::invoke;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::{prelude::Redirect, Routable};

#[derive(Clone, Routable, PartialEq, Eq)]
pub enum Route {
	#[at("/settings")]
	Settings,
	#[at("/settings/friends")]
	AddAccount,
	#[not_found]
	#[at("/settings/404")]
	NotFound,
}

pub fn switch_route(route: Route) -> Html {
	match route {
		Route::Settings => html! {<Settings></Settings>},
		Route::AddAccount => html! { <AddAccount></AddAccount> },
		Route::NotFound => {
			html! {<Redirect<super::Route> to={super::Route::Peeps}/>}
		}
	}
}

#[function_component(AddAccount)]
fn add_account() -> Html {
	let on_login = Callback::from(move |ev: SubmitEvent| {
		// Prevent actual form submission
		ev.prevent_default();
	});

	let platform_options = onlivfe::platforms()
		.into_iter()
		.map(|platform| {
			html! {
				<option value={platform.as_ref().to_string()}>{platform.as_ref().to_string()}</option>
			}
		})
		.collect::<Html>();

	html! {
		<>
			<h2>{"Add account"}</h2>
			<form onsubmit={on_login}>
				<select>
					{ platform_options }
				</select>
				<input type="text" placeholder="Login"/>
				<input type="password" placeholder="Password" />
				<button type="submit">{"Login"}</button>
			</form>
		</>
	}
}

#[function_component(AccountsList)]
fn accounts_list() -> Html {
	let authenticated_accounts = use_async_with_options(
		async move {
			match invoke::<_, Vec<PlatformAccount>>("authenticated_accounts", &())
				.await
			{
				Ok(p) => Ok(p),
				Err(e) => Err(e.to_string()),
			}
		},
		UseAsyncOptions::enable_auto(),
	);

	if authenticated_accounts.loading {
		html! {
			<section>
				<h2>{"Accounts"}</h2>
				<progress></progress>
			</section>
		}
	} else if let Some(error) = authenticated_accounts.error.as_ref() {
		html! {
			<section>
				<h2>{"Error loading accounts"}</h2>
				<details>
					<summary>{"Details"}</summary>
					<code>{error}</code>
				</details>
			</section>
		}
	} else {
		html! {
			<section>
				<h2>{ "Accounts" }</h2>
				<ul>
						{ authenticated_accounts.data.as_ref().map_or_else(
							|| html!{<p>{"Failed to get accounts"}</p>},
							|v| v.iter().map(|acc| html!{ <li>{acc.id().id_as_string()}</li> }).collect::<Html>()
						)}
				</ul>
			</section>
		}
	}
}

#[function_component(Settings)]
fn settings() -> Html {
	html! {
		<>
			<h1>{"Settings"}</h1>
			<AccountsList/>
			<AddAccount/>
		</>
	}
}
