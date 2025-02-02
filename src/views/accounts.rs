use dioxus::prelude::*;
use dioxus_i18n::tid;
use onlivfe::{LoginCredentials, PlatformType, cvr, resonite, vrchat};

//const LOGIN_CSS: Asset = asset!("/res/css/login.css");

fn default_login_credentials(platform: PlatformType) -> LoginCredentials {
	match platform {
		PlatformType::VRChat => LoginCredentials::VRChat(Box::new(
			vrchat::LoginRequestPart::LoginRequest(vrchat::query::Authenticating {
				username: String::new(),
				password: String::new(),
			}),
		)),
		PlatformType::ChilloutVR => {
			LoginCredentials::ChilloutVR(Box::new(cvr::query::LoginCredentials {
				email: String::new(),
				password: String::new(),
			}))
		}
		PlatformType::Resonite => LoginCredentials::Resonite(Box::new(
			resonite::query::UserSessionQueryWithHeaders {
				body: resonite::query::UserSession {
					authentication: resonite::query::UserSessionAuthentication::Password(
						resonite::query::UserSessionPasswordAuthentication {
							password: String::new(),
							recovery_code: None,
						},
					),
					identifier: resonite::query::LoginCredentialsIdentifier::Email(
						String::new(),
					),
					remember_me: true,
					secret_machine_id: String::new(),
				},
				data: resonite::query::Authenticating {
					second_factor: None,
					unique_machine_identifier: String::new(),
				},
			},
		)),
	}
}

#[component]
pub fn Accounts() -> Element {
	use strum::VariantNames;

	let mut totp_enabled = use_signal(|| true);
	let mut login_processing = use_signal(|| false);
	let mut login_data =
		use_signal(|| default_login_credentials(PlatformType::VRChat));

	let selected_platform = use_memo(move || login_data().platform());

	let accounts_len = use_signal(|| 0);

	let login_request = use_resource(move || async move {
		if !login_processing() {
			return None;
		}
		let creds = login_data();
		Some(login_server(creds).await)
	});

	use_effect(move || {
		if login_request().is_some() && login_processing() {
			println!("Login request done, resetting");
			*login_processing.write() = false;
		}
	});

	let form_disable = use_memo(move || {
		login_processing() || (login_request.state() == UseResourceState::Pending)
	});

	// TODO: Translate
	rsx! {
		section { id: "login",

			// Content
			h1 { {tid!("accounts")} }

			p { {tid!("logged-in-accounts-count", count : accounts_len())} }

			section {
				h2 { {tid!("add-account")} }

				form {
					onsubmit: move |e| {
					    e.prevent_default();
					    *login_processing.write() = true;
					},
					fieldset { role: "group", disabled: form_disable(),
						select {
							name: "platform",
							aria_label: tid!("select-platform-type"),
							for platform in onlivfe::platforms() {
								option {
									selected: selected_platform() == platform,
									onclick: move |_| async move {
									    login_data.set(default_login_credentials(platform));
									},
									{tid!(& platform.as_ref().to_ascii_lowercase())}
								}
							}
						}
						if selected_platform() == onlivfe::PlatformType::Resonite {
							select {
								name: "identifier-type",
								aria_label: tid!("select-identifier-type"),
								disabled: true,
								for id_type in onlivfe::resonite::query::LoginCredentialsIdentifier::VARIANTS {
									option { selected: id_type == &"Email",
										{tid!(& id_type.to_ascii_lowercase())}
									}
								}
							}
						} else {
							select {
								name: "identifier-type",
								aria_label: "Identifier type",
								disabled: true,
								option { {tid!("email")} }
							}
						}
					}
					fieldset { role: "group", disabled: form_disable(),
						input {
							name: "user",
							autocomplete: "email",
							required: true,
							r#type: "email",
							placeholder: tid!("email"),
						}
						input {
							name: "password",
							r#type: "password",
							required: true,
							placeholder: tid!("password"),
						}
					}
					fieldset { role: "group", disabled: form_disable(),
						label { width: "fit-content", text_wrap: "nowrap",
							input {
								r#type: "checkbox",
								checked: totp_enabled(),
								role: "switch",
								onclick: move |_| async move {
								    totp_enabled.set(!totp_enabled());
								},
							}
							{tid!("enable-totp")}
						}
						if totp_enabled() {
							input {
								name: "totp",
								r#type: "number",
								min: 0,
								max: 999999,
								step: 1,
								required: true,
								pattern: "\\d{6}",
								placeholder: tid!("totp"),
							}
						}
						button {
							r#type: "submit",
							width: "100%",
							aria_busy: form_disable(),
							{tid!("login")}
						}
					}
				}
			}
		}
	}
}

/// Echo the user input on the server.
#[server(LoginServer)]
async fn login_server(creds: LoginCredentials) -> Result<(), ServerFnError> {
	eprintln!("Req!");
	Ok(())
}
