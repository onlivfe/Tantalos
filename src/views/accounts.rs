use dioxus::prelude::*;

//const LOGIN_CSS: Asset = asset!("/res/css/login.css");

#[component]
pub fn Accounts() -> Element {
	let mut response = use_signal(String::new);

	rsx! {
			//document::Link { rel: "stylesheet", href: LOGIN_CSS}

			section {
					id: "login",

					// Content
					h1 { "Accounts" }

								div {
					id: "echo",
					h4 { "ServerFn Echo" }
					input {
							placeholder: "Type here to echo...",
							oninput:  move |event| async move {
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
