use onlivfe::PlatformAccount;
use tauri_sys::tauri::invoke;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

mod add_account;
use add_account::*;

#[derive(Clone, Routable, PartialEq, Eq)]
pub enum Route {
	#[at("/settings")]
	Settings,
	#[at("/settings/add-account")]
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
			<Link<Route> to={Route::AddAccount} classes="button"> {"Add account"} </Link<Route>>
		</>
	}
}
