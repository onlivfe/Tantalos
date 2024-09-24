use leptos::{html::Form, *};
use onlivfe::{
	LoginCredentials,
	LoginError,
	PlatformAccountId,
	PlatformType,
	resonite::query::UserSessionAuthentication,
};
use tauri_sys::tauri::invoke;
use web_sys::{Event, SubmitEvent};

fn switch_login_credentials_platform(
	old: Option<&LoginCredentials>, new_platform: PlatformType,
) -> LoginCredentials {
	let old_name = match old {
		Some(LoginCredentials::VRChat(boxed)) => match &**boxed {
			onlivfe::vrchat::LoginRequestPart::LoginRequest(v) => v.username.clone(),
			onlivfe::vrchat::LoginRequestPart::SecondFactor(_) => String::new(),
		},
		Some(LoginCredentials::ChilloutVR(v)) => v.email.clone(),
		Some(LoginCredentials::Resonite(v)) => match v.body.identifier.clone() {
			onlivfe::resonite::query::LoginCredentialsIdentifier::Email(v)
			| onlivfe::resonite::query::LoginCredentialsIdentifier::Username(v)
			| onlivfe::resonite::query::LoginCredentialsIdentifier::OwnerID(v) => v,
		},
		_ => String::new(),
	};

	let old_pw = match old {
		Some(LoginCredentials::VRChat(boxed)) => match &**boxed {
			onlivfe::vrchat::LoginRequestPart::LoginRequest(v) => v.password.clone(),
			onlivfe::vrchat::LoginRequestPart::SecondFactor(_) => String::new(),
		},
		Some(LoginCredentials::ChilloutVR(v)) => v.password.clone(),
		Some(LoginCredentials::Resonite(v)) => match v.body.authentication {
			UserSessionAuthentication::Password(pw) => pw.password.clone(),
			_ => "".to_owned()
		},
		_ => String::new(),
	};

	match new_platform {
		PlatformType::VRChat => LoginCredentials::VRChat(Box::new(
			onlivfe::vrchat::LoginRequestPart::LoginRequest(
				onlivfe::vrchat::query::Authenticating {
					username: old_name,
					password: old_pw,
				},
			),
		)),
		PlatformType::ChilloutVR => LoginCredentials::ChilloutVR(Box::new(
			onlivfe::cvr::query::LoginCredentials {
				email: old_name,
				password: old_pw,
			},
		)),
		PlatformType::Resonite => LoginCredentials::Resonite(Box::new(
			onlivfe::resonite::query::LoginCredentials::new(
				onlivfe::resonite::query::LoginCredentialsIdentifier::Username(old_name),
				old_pw,
			),
		)),
	}
}

async fn request_login(
	credentials: LoginCredentials,
) -> Result<onlivfe::PlatformAccountId, onlivfe::LoginError> {
	#[derive(serde::Serialize)]
	struct LoginArgs {
		credentials: LoginCredentials,
	}
	let credentials = credentials.clone();
	match invoke::<LoginArgs, Result<PlatformAccountId, LoginError>>(
		"login",
		&LoginArgs { credentials },
	)
	.await
	{
		Ok(res) => res,
		Err(e) => Err(LoginError::Error(e.to_string())),
	}
}

#[component]
pub fn add_account() -> impl IntoView {
	let (credentials, credentials_setter) = create_signal(
		switch_login_credentials_platform(None, PlatformType::VRChat),
	);

	let login = create_action(|credentials: &LoginCredentials| {
		request_login(credentials.clone())
	});

	let form_element: NodeRef<Form> = create_node_ref();

	let on_login = move |event: SubmitEvent| {
		// Prevent actual form submission
		event.prevent_default();

		// Yes we're only doing a client side check, not best practice but meh
		if form_element.get().unwrap().report_validity() && !login.pending().get() {
			login.dispatch(credentials.get());
		}
	};

	match login.value().get() {
		None => view! {
			<h1>"Logging in..."</h1>
			<progress></progress>
		}
		.into_view(),
		Some(Err(error)) => match error {
			LoginError::Error(error) => view! {
				<section>
					<h1>{"Error logging in"}</h1>
					<details>
						<summary>{"Details"}</summary>
						<code>{error}</code>
					</details>
				</section>
			}
			.into_view(),
			LoginError::RequiresAdditionalFactor(_) => view! {
				<h1>{"Requires 2FA"}</h1>
				<section>
					<details>
						<code>{"Not implemented yet"}</code>
					</details>
				</section>
			}
			.into_view(),
		},
		Some(Ok(_)) => view! {
			<h1>{"Add account"}</h1>
			<form on:submit=on_login>
				<AccountPlatformSelector
					value=Signal::derive(move || credentials.get().platform())
					on_change=move |event| {
						use std::str::FromStr;
						if let Ok(platform) = PlatformType::from_str(&event_target_value(&event)) {
							credentials_setter
								.set(
									switch_login_credentials_platform(
										Some(&credentials.get()),
										platform,
									),
								);
						}
					}
				/>

				<AccountCredentialsInput value=credentials setter=credentials_setter/>
				<button type="submit">{"Login"}</button>
			</form>
		}
		.into_view(),
	}
}

#[component]
fn account_platform_selector<F>(
	#[prop(into)] value: Signal<PlatformType>, on_change: F,
) -> impl IntoView
where
	F: Fn(Event) + 'static,
{
	let platform_options = onlivfe::platforms()
		.into_iter()
		.map(|platform| {
			view! {
				<option
					value=platform.as_ref().to_string()
					selected=move || value.get() == platform
				>
					{platform.as_ref().to_string()}
				</option>
			}
		})
		.collect::<Vec<_>>();

	view! { <select on:change=on_change>{platform_options}</select> }
}

#[component]
fn account_credentials_input(
	#[prop(into)] value: Signal<LoginCredentials>,
	#[prop(into)] setter: WriteSignal<LoginCredentials>,
) -> impl IntoView {
	match value.get() {
		LoginCredentials::VRChat(credentials) => {
			let (value, vrc_setter) = create_signal(*credentials);
			create_isomorphic_effect(move |_| {
				setter.set(LoginCredentials::VRChat(Box::new(value.get())));
			});

			view! { <VrcAccountCredentialsInput value=value setter=vrc_setter/> }
				.into_view()
		}
		LoginCredentials::ChilloutVR(credentials) => {
			let (value, cvr_setter) = create_signal(*credentials);
			create_isomorphic_effect(move |_| {
				setter.set(LoginCredentials::ChilloutVR(Box::new(value.get())));
			});

			view! { <CvrAccountCredentialsInput value=value setter=cvr_setter/> }
				.into_view()
		}
		LoginCredentials::Resonite(credentials) => {
			let (value, neos_setter) = create_signal(*credentials);
			create_isomorphic_effect(move |_| {
				setter.set(LoginCredentials::Resonite(Box::new(value.get())));
			});

			view! { <NeosAccountCredentialsInput value=value setter=neos_setter/> }
				.into_view()
		}
	}
}

#[component]
fn vrc_account_credentials_input(
	#[prop(into)] value: Signal<onlivfe::vrchat::LoginRequestPart>,
	#[prop(into)] setter: WriteSignal<onlivfe::vrchat::LoginRequestPart>,
) -> impl IntoView {
	use onlivfe::vrchat::LoginRequestPart;

	match value.get() {
		LoginRequestPart::LoginRequest(credentials) => {
			let on_username_change = {
				let creds = credentials.clone();
				move |event: Event| {
					let mut creds = creds.clone();
					creds.username = event_target_value(&event);
					setter.set(LoginRequestPart::LoginRequest(creds));
				}
			};

			let on_password_change = {
				let creds = credentials.clone();
				move |event: Event| {
					let mut creds = creds.clone();
					creds.password = event_target_value(&event);
					setter.set(LoginRequestPart::LoginRequest(creds));
				}
			};

			view! {
				<input
					required=true
					type="text"
					placeholder="Username"
					prop:value=credentials.username.clone()
					on:change=on_username_change
				/>
				<input
					required=true
					type="password"
					placeholder="Password"
					prop:value=credentials.password.clone()
					on:change=on_password_change
				/>
			}
			.into_view()
		}
		LoginRequestPart::SecondFactor((id, second_factor)) => {
			use onlivfe::vrchat::query::VerifySecondFactor;
			match second_factor {
				VerifySecondFactor::Code(totp) => {
					let on_totp_change = move |event: Event| {
						setter.set(LoginRequestPart::SecondFactor((
							id.clone(),
							VerifySecondFactor::Code(event_target_value(&event)),
						)));
					};

					view! {
						<input
							required=true
							type="number"
							placeholder="2FA"
							prop:value=totp
							on:change=on_totp_change
						/>
					}
					.into_view()
				}
				VerifySecondFactor::Email(email_otp) => {
					let on_totp_change = move |event: Event| {
						setter.set(LoginRequestPart::SecondFactor((
							id.clone(),
							VerifySecondFactor::Email(event_target_value(&event)),
						)));
					};

					view! {
						<input
							required=true
							type="number"
							placeholder="Email code"
							prop:value=email_otp
							on:change=on_totp_change
						/>
					}
					.into_view()
				}
				VerifySecondFactor::Recovery(recovery) => {
					let on_totp_change = move |event: Event| {
						setter.set(LoginRequestPart::SecondFactor((
							id.clone(),
							VerifySecondFactor::Recovery(event_target_value(&event)),
						)));
					};

					view! {
						<input
							required=true
							type="number"
							placeholder="Recovery code"
							prop:prop:value=recovery
							on:change=on_totp_change
						/>
					}
					.into_view()
				}
			}
		}
	}
}

#[component]
fn cvr_account_credentials_input(
	#[prop(into)] value: Signal<onlivfe::cvr::query::LoginCredentials>,
	#[prop(into)] setter: WriteSignal<onlivfe::cvr::query::LoginCredentials>,
) -> impl IntoView {
	let on_email_change = move |event: Event| {
		let mut creds = value.get();
		creds.email = event_target_value(&event);
		setter.set(creds);
	};

	let on_password_change = move |event: Event| {
		let mut creds = value.get();
		creds.password = event_target_value(&event);
		setter.set(creds);
	};

	view! {
		<input
			required=true
			type="email"
			placeholder="Email"
			prop:value=value.get().email
			on:change=on_email_change
		/>
		<input
			required=true
			type="password"
			placeholder="Password"
			prop:value=value.get().password
			on:change=on_password_change
		/>
	}
}

#[component]
fn neos_account_credentials_input(
	#[prop(into)] value: Signal<onlivfe::resonite::query::LoginCredentials>,
	#[prop(into)] setter: WriteSignal<onlivfe::resonite::query::LoginCredentials>,
) -> impl IntoView {
	use onlivfe::resonite::query::LoginCredentialsIdentifier;

	let on_identifier_type_change = move |event: Event| {
		let mut creds = value.get();
		creds.identifier = match event_target_value(&event).as_ref() {
			"Username" => LoginCredentialsIdentifier::Username(
				creds.identifier.inner().to_string(),
			),
			"Email" => {
				LoginCredentialsIdentifier::Email(creds.identifier.inner().to_string())
			}
			_ => LoginCredentialsIdentifier::OwnerID(
				creds.identifier.inner().to_string(),
			),
		};

		setter.set(creds);
	};
	let identifier_picker = view! {
		<select on:change=on_identifier_type_change>
			<option value="Username">{"Username"}</option>
			<option value="Email">{"Email"}</option>
			<option value="OwnerId">{"User ID"}</option>
		</select>
	};

	let identifier_html = match value.get().identifier {
		LoginCredentialsIdentifier::OwnerID(user_id) => {
			let on_change = move |event: Event| {
				let mut creds = value.get();
				creds.identifier =
					LoginCredentialsIdentifier::OwnerID(event_target_value(&event));
				setter.set(creds);
			};

			view! {
				<input
					required=true
					type="text"
					placeholder="User-ID"
					prop:value=user_id
					on:input=on_change
				/>
			}
		}
		LoginCredentialsIdentifier::Email(email) => {
			let on_change = move |event: Event| {
				let mut creds = value.get();
				creds.identifier =
					LoginCredentialsIdentifier::Email(event_target_value(&event));
				setter.set(creds);
			};

			view! { <input required=true type="text" placeholder="Email" prop:value=email on:input=on_change/> }
		}
		LoginCredentialsIdentifier::Username(username) => {
			let on_change = move |event: Event| {
				let mut creds = value.get();
				creds.identifier =
					LoginCredentialsIdentifier::Username(event_target_value(&event));
				setter.set(creds);
			};
			view! {
				<input
					required=true
					type="text"
					placeholder="Username"
					prop:value=username
					on:input=on_change
				/>
			}
		}
	};

	let on_password_change = move |event: Event| {
		let mut creds = value.get();
		creds.password = event_target_value(&event);
		setter.set(creds);
	};

	let on_totp_change = move |event: Event| {
		let mut creds = value.get();
		let val = event_target_value(&event);
		creds.totp = if val.is_empty() { None } else { Some(val) };
		setter.set(creds);
	};

	view! {
		<ul style="flex-direction: row; flex-wrap: wrap;">
			<li style="flex-grow: 1;">{identifier_picker}</li>
			<li style="flex-grow: 5;">{identifier_html}</li>
		</ul>
		<ul style="flex-direction: row; flex-wrap: wrap;">
			<li style="flex-grow: 8;">
				<input
					required=true
					type="password"
					placeholder="Password"
					prop:value=value.get().password
					on:input=on_password_change
				/>
			</li>
			<li style="flex-grow: 1;">
				<input
					required=false
					type="number"
					placeholder="2FA"
					prop:value=value.get().totp
					on:input=on_totp_change
				/>
			</li>
		</ul>
	}
}
