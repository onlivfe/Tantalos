use dioxus::{html::h2, prelude::*};
use dioxus_i18n::t;

//const LOGIN_CSS: Asset = asset!("/res/css/login.css");

#[component]
pub fn Accounts() -> Element {
	use strum::VariantNames;

	let mut totp_enabled = use_signal(|| true);
	let mut response = use_signal(String::new);
	let mut selected_platform = use_signal(|| onlivfe::PlatformType::VRChat);

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
						select {
							name: "platform",
							aria_label: "Select platform",
							for platform in onlivfe::platforms() {
								option {
									selected: selected_platform() == platform,
									onclick: move |_| async move {
											selected_platform.set(platform);
									},
									{ platform.as_ref() }
								}
							}
						},
						if selected_platform() == onlivfe::PlatformType::Resonite {
							select {
								name: "identifier-type",
								aria_label: "Identifier type",
								disabled: true,
								for id_type in onlivfe::resonite::query::LoginCredentialsIdentifier::VARIANTS {
									option {
										selected: id_type == &"Email",
										{ id_type }
									}
								}
							},
						} else {
							select {
								name: "identifier-type",
								aria_label: "Identifier type",
								disabled: true,
								option {
									{ "Email" }
								}
							},
						}
					}
					fieldset {
						role: "group",
						input {
							name: "user",
							autocomplete: "email",
							required: true,
							type: "email",
							placeholder: "Email"
						},
						input {
							name: "password",
							type: "password",
							required: true,
							placeholder: "Password"
						},
					}
					fieldset {
						role: "group",
						label {
							width: "fit-content",
							text_wrap: "nowrap",
							input {
								type: "checkbox",
								checked: totp_enabled(),
								role: "switch",
								onclick: move |_| async move {
									totp_enabled.set(!totp_enabled());
								},
							},
							{ "Use TOTP" }
						}
						if totp_enabled() {
							input {
								name: "totp",
								type: "number",
								min: 0,
								max: 999999,
								step: 1,
								required: true,
								pattern: "\\d{6}",
								placeholder: "TOTP"
							},
						}
						button {
							type: "submit",
							aria_busy: true,
							disabled: true,
							{ "Log in" }
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
