use dioxus::{html::h2, prelude::*};
use dioxus_i18n::t;

//const LOGIN_CSS: Asset = asset!("/res/css/login.css");

#[component]
pub fn Accounts() -> Element {
	let mut response = use_signal(String::new);

	let accounts_len = 0;

	// TODO: Translate
	rsx! {
		section { id: "login",

			// Content
			h1 { { t!("accounts") } }

			p {  {t!("logged-in-accounts-count", count: accounts_len)} }

			section {
				h2 { "Add account"}

				form {
					fieldset {
						role: "group",
						input {
							name: "user",
							autocomplete: "email",
							type: "text",
							placeholder: "Log in"
						},
						input {
							name: "password",
							type: "password",
							placeholder: "Password"
						},
						input {
							type: "submit",
							value: "Log in"
						}
					}
				}
			}

			div { id: "echo",
				h4 { "ServerFn Echo" }
				input {
					placeholder: "Type here to echo...",
					oninput: move |event| async move {
							let data = login_server(event.value()).await.unwrap();
							response.set(data);
					},
				}

				if !response().is_empty() {
					p {
						"Server echoed: "
						i { "{response}" }
					}
				}
			}
		}
	}
}

/// Echo the user input on the server.
#[server(LoginServer)]
async fn login_server(input: String) -> Result<String, ServerFnError> {
	Ok(input.clone() + &input)
}
