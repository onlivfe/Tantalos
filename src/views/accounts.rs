use dioxus::prelude::*;
use onlivfe::{cvr, resonite, vrchat, LoginCredentials, LoginError, PlatformAccountId, PlatformType};
use tracing::{info, trace};

use crate::tid;

fn default_login_credentials(platform: PlatformType) -> LoginCredentials {
	match platform {
		PlatformType::VRChat => LoginCredentials::VRChat(Box::new(
			vrchat::LoginRequestPart::LoginRequest(vrchat::query::Authenticating {
				username: if cfg!(debug_assertions) { "dev@example.org".to_owned()} else { String::new() },
				password: if cfg!(debug_assertions) { "example".to_owned()} else { String::new() },
			}),
		)),
		PlatformType::ChilloutVR => {
			LoginCredentials::ChilloutVR(Box::new(cvr::query::LoginCredentials {
				email: if cfg!(debug_assertions) { "dev@example.org".to_owned()} else { String::new() },
				password: if cfg!(debug_assertions) { "example".to_owned()} else { String::new() },
			}))
		}
		PlatformType::Resonite => LoginCredentials::Resonite(Box::new(
			resonite::query::UserSessionQueryWithHeaders {
				body: resonite::query::UserSession {
					authentication: resonite::query::UserSessionAuthentication::Password(
						resonite::query::UserSessionPasswordAuthentication {
							password: if cfg!(debug_assertions) { "example".to_owned()} else { String::new() },
							recovery_code: None,
						},
					),
					identifier: resonite::query::LoginCredentialsIdentifier::Email(
						if cfg!(debug_assertions) { "dev@example.org".to_owned()} else { String::new() }
					),
					remember_me: true,
					secret_machine_id: String::new(),
				},
				data: resonite::query::Authenticating {
					second_factor: if cfg!(debug_assertions) { Some("123456".to_owned()) } else { None },
					unique_machine_identifier: String::new(),
				},
			},
		)),
	}
}


#[component]
pub fn Accounts() -> Element {
	let raw_account_ids =  use_server_future(get_accounts_from_server)?;
	let account_ids = use_memo(move || {
		let ids = raw_account_ids();
		trace!("{ids:?}");
		match ids {
		Some(Ok(Ok(v))) => v,
		_ => vec![]
	}
	});
	let accounts_len = use_memo(move || account_ids().len());

	rsx! {
		section { id: "login",

			// Content
			hgroup {
				h1 { {tid!("accounts")} }
				p { {tid!("logged-in-accounts-count", count : accounts_len())} }
				div {
					class: "grid",
					for account_id in account_ids() {
						article {
							{ account_id.id_as_string() }
						}
					}
				}
			}


			section {
				h2 { {tid!("add-account")} }
				LoginForm {}
			}
		}
	}
}

#[component]
fn LoginForm() -> Element {
	use strum::VariantNames;

	let mut login_processing = use_signal(|| false);
	let mut login_data =
		use_signal(|| default_login_credentials(PlatformType::VRChat));

	let selected_platform = use_memo(move || login_data().platform());

	let login_request = use_resource(move || async move {
		if !login_processing() {
			return None;
		}
		let creds = login_data();
		Some(login_server(creds).await)
	});

	let login_needs_reset =
		use_memo(move || login_request().is_some() && login_processing());
	use_effect(move || {
		if login_needs_reset() {
			trace!("Login request done, resetting");
			*login_processing.write() = false;
		}
	});

	let form_disable = use_memo(move || {
		login_processing() || (login_request.state() == UseResourceState::Pending)
	});
	let primary_secret_supported = use_memo(move || match login_data() {
			LoginCredentials::VRChat(login_request_part) => match &*login_request_part {
					vrchat::LoginRequestPart::LoginRequest(_) => true,
					vrchat::LoginRequestPart::SecondFactor(_) => false,
			},
			LoginCredentials::ChilloutVR(_) => true,
			LoginCredentials::Resonite(_) => true,
	});
	let secondary_secret_supported = use_memo(move || match login_data() {
			LoginCredentials::VRChat(login_request_part) => match &*login_request_part {
					vrchat::LoginRequestPart::LoginRequest(_) => false,
					vrchat::LoginRequestPart::SecondFactor(_) => true,
			},
			LoginCredentials::ChilloutVR(_) => false,
			LoginCredentials::Resonite(_) => true,
	});

	let login_id =
		use_memo(move || login_data().identifier().to_owned());
	let primary_secret =
		use_memo(move || login_data().primary_secret().unwrap_or_default().to_owned());
	let secondary_secret_opt =
		use_memo(move || login_data().secondary_secret().map(|v| v.to_owned()));
	let secondary_secret =
		use_memo(move || secondary_secret_opt().unwrap_or_default().to_owned());

	rsx! {
		form {
			method: "dialog",
			onsubmit: move |e| {
			    info!("Login submitted");
			    e.prevent_default();
			    *login_processing.write() = true;
			},
			if form_disable() {
				progress {}
			}
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
					value: "{login_id}",
					oninput: move |event| {
					    let value = event.value();
					    if let Err(e) = login_data.with_mut(|d| { d.set_identifier(value) }) {
					        tracing::error!("Modifying identifier failed: {:?}", e);
					    }
					},
				}
				input {
					name: "password",
					r#type: "password",
					required: primary_secret_supported(),
					disabled: !primary_secret_supported(),
					placeholder: tid!("password"),
					value: "{primary_secret}",
					oninput: move |event| {
					    let value = event.value();
					    if let Err(e) = login_data.with_mut(|d| { d.set_primary_secret(value) }) {
					        tracing::error!("Modifying primary secret failed: {:?}", e);
					    }
					},
				}
			}
			fieldset {
				align_items: "baseline",
				class: "grid",
				disabled: form_disable(),
				if secondary_secret_supported() {
					label {
						input {
							r#type: "checkbox",
							checked: secondary_secret_opt().is_some(),
							role: "switch",
							onchange: move |event| async move {
							    let value = event.checked();
									let value = { if value { Some("".to_owned()) } else {None}};
							    if let Err(e) = login_data.with_mut(|d| { d.set_secondary_secret(value) }) {
							        tracing::error!("Modifying primary secret failed: {:?}", e);
							    }
							},
						}
						{tid!("enable-totp")}
					}
					if secondary_secret_opt().is_some() {
						input {
							name: "totp",
							r#type: "number",
							min: 0,
							max: 999999,
							step: 1,
							required: secondary_secret_supported(),
							disabled: !secondary_secret_supported(),
							pattern: "\\d{6}",
							placeholder: tid!("totp"),
							value: "{secondary_secret}",
							oninput: move |event| {
							    let value = event.value();
							    if let Err(e) = login_data.with_mut(|d| { d.set_secondary_secret(Some(value)) }) {
							        tracing::error!("Modifying primary secret failed: {:?}", e);
							    }
							},
						}
					}
				}
				button { r#type: "submit", aria_busy: form_disable(), {tid!("login")} }
			}
		}
	}
}

/// Echo the user input on the server.
#[server(LoginServer)]
async fn login_server(creds: LoginCredentials) -> Result<Result<PlatformAccountId, LoginError>, ServerFnError> {
	let FromContext(onlivfe): FromContext<crate::Onlivfe> = extract().await?;
	let res = onlivfe.login(creds).await;
	if let Err(err) = &res {
		tracing::error!("Login failed {err}");
	}
	Ok(res)
}

#[server(AccountsServer)]
async fn get_accounts_from_server() -> Result<Result<Vec<PlatformAccountId>, String>, ServerFnError> {
	let FromContext(onlivfe): FromContext<crate::Onlivfe> = extract().await?;
	let res = onlivfe.authenticated_accounts().await;
	if let Err(err) = &res {
		tracing::error!("Getting authenticated accounts failed {err}");
	}
	Ok(res)
}
