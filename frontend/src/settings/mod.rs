use leptos::*;
use leptos_router::*;
use onlivfe::PlatformAccount;
use tauri_sys::tauri::invoke;

mod add_account;
pub use add_account::AddAccount;

use crate::SettingsPage;

#[component]
fn accounts_list() -> impl IntoView {
	let authenticated_accounts = create_resource(
		|| (),
		|()| async move {
			match invoke::<_, Vec<PlatformAccount>>("authenticated_accounts", &())
				.await
			{
				Ok(p) => Ok(p),
				Err(e) => Err(e.to_string()),
			}
		},
	);

	match authenticated_accounts.get() {
		None => view! {
			<section>
				<h2>{"Accounts"}</h2>
				<progress></progress>
			</section>
		}
		.into_view(),
		Some(Err(error)) => view! {
			<section>
				<h2>{"Error loading accounts"}</h2>
				<details>
					<summary>{"Details"}</summary>
					<code>{error}</code>
				</details>
			</section>
		}
		.into_view(),
		Some(Ok(authenticated_accounts)) => view! {
			<section>
				<h2>{"Accounts"}</h2>
				<ul>
					<For
						each=move || authenticated_accounts.clone()
						key=PlatformAccount::id
						children=move |acc| {
							view! { <li>{acc.id().id_as_string()}</li> }
						}
					/>

				</ul>
			</section>
		}
		.into_view(),
	}
}

#[component]
pub fn settings() -> impl IntoView {
	view! {
		<h1>{"Settings"}</h1>
		<AccountsList/>
		<A href=SettingsPage::AddUser.path() class="button">
			{"Add account"}
		</A>
	}
}
