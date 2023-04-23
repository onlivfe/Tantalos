use onlivfe::{LoginCredentials, LoginError, PlatformAccountId, PlatformType};
use tauri_sys::tauri::invoke;
use web_sys::{HtmlFormElement, HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::TwoWayBindingProps;

fn switch_login_credentials_platform(
	old: Option<&LoginCredentials>, new_platform: PlatformType,
) -> LoginCredentials {
	let old_name = match old {
		Some(LoginCredentials::VRChat(boxed)) => match &**boxed {
			onlivfe::vrchat::LoginRequestPart::LoginRequest(v) => v.username.clone(),
			onlivfe::vrchat::LoginRequestPart::SecondFactor(_) => String::new(),
		},
		Some(LoginCredentials::ChilloutVR(v)) => v.email.clone(),
		Some(LoginCredentials::NeosVR(v)) => match v.identifier.clone() {
			onlivfe::neosvr::query::LoginCredentialsIdentifier::Email(v)
			| onlivfe::neosvr::query::LoginCredentialsIdentifier::Username(v)
			| onlivfe::neosvr::query::LoginCredentialsIdentifier::OwnerID(v) => v,
		},
		_ => String::new(),
	};

	let old_pw = match old {
		Some(LoginCredentials::VRChat(boxed)) => match &**boxed {
			onlivfe::vrchat::LoginRequestPart::LoginRequest(v) => v.password.clone(),
			onlivfe::vrchat::LoginRequestPart::SecondFactor(_) => String::new(),
		},
		Some(LoginCredentials::ChilloutVR(v)) => v.password.clone(),
		Some(LoginCredentials::NeosVR(v)) => v.password.clone(),
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
		PlatformType::NeosVR => LoginCredentials::NeosVR(Box::new(
			onlivfe::neosvr::query::LoginCredentials::new(
				onlivfe::neosvr::query::LoginCredentialsIdentifier::Username(old_name),
				old_pw,
			),
		)),
	}
}

#[function_component(AddAccount)]
pub fn add_account() -> Html {
	let credentials_state = use_state_eq(|| {
		switch_login_credentials_platform(None, PlatformType::VRChat)
	});

	let login: UseAsyncHandle<PlatformAccountId, LoginError> = {
		let credentials_state = credentials_state.clone();
		use_async(async move {
			#[derive(serde::Serialize)]
			struct Args<'a> {
				credentials: &'a LoginCredentials,
			}

			match invoke::<_, Result<PlatformAccountId, LoginError>>(
				"login",
				&Args { credentials: &credentials_state },
			)
			.await
			{
				Ok(res) => res,
				Err(e) => Err(LoginError::Error(e.to_string())),
			}
		})
	};

	let on_login = {
		let login = login.clone();
		Callback::from(move |event: SubmitEvent| {
			// Prevent actual form submission
			event.prevent_default();
			let form = event.target_unchecked_into::<HtmlFormElement>();

			// Yes we're only doing a client side check, not best practice but meh
			if form.report_validity() && !login.loading {
				login.run();
			}
		})
	};

	let on_platform_change = {
		let credentials_state = credentials_state.clone();
		Callback::from(move |platform: PlatformType| {
			credentials_state.set(switch_login_credentials_platform(
				Some(&*credentials_state),
				platform,
			));
		})
	};

	let on_credentials_change = {
		let credentials_state = credentials_state.clone();
		Callback::from(move |credentials: LoginCredentials| {
			credentials_state.set(credentials);
		})
	};

	if let Some(error) = login.error.as_ref() {
		match error {
			LoginError::Error(error) => {
				html! {
				<section>
					<h1>{"Error logging in"}</h1>
					<details>
						<summary>{"Details"}</summary>
						<code>{error}</code>
					</details>
				</section>
			}
		},
		LoginError::RequiresAdditionalFactor(id) => {
				html!{
					<section>
					<h1>{"Requires 2FA"}</h1>
					<details>
						<code>{"Not implemented yet"}</code>
					</details>
				</section>
				}
			}
		}
	} else {
		html! {
			<>
				<h1>{"Add account"}</h1>
				<form onsubmit={on_login}>
					<fieldset disabled={login.loading}>
						<AccountPlatformSelector value={credentials_state.platform()} onchange={on_platform_change}  />
						<AccountCredentialsInput value={(*credentials_state).clone()} onchange={on_credentials_change} />
						<button type="submit">{"Login"}</button>
					</fieldset>
				</form>
			</>
		}
	}
}

#[function_component(AccountPlatformSelector)]
fn account_platform_selector(props: &TwoWayBindingProps<PlatformType>) -> Html {
	let onchange = {
		let cb = props.onchange.clone();

		Callback::from(move |event: Event| {
			use std::str::FromStr;

			let select = event.target_unchecked_into::<HtmlSelectElement>();
			if let Ok(platform) = PlatformType::from_str(&select.value()) {
				cb.emit(platform);
			}
		})
	};

	let platform_options = onlivfe::platforms()
		.into_iter()
		.map(|platform| {
			html! {
				<option value={platform.as_ref().to_string()} selected={props.value == platform}>{platform.as_ref().to_string()}</option>
			}
		})
		.collect::<Html>();

	html! {
		<select onchange={onchange}>
			{platform_options}
		</select>
	}
}

#[function_component(AccountCredentialsInput)]
fn account_credentials_input(
	props: &TwoWayBindingProps<LoginCredentials>,
) -> Html {
	match &props.value {
		LoginCredentials::VRChat(credentials) => {
			let cb = {
				let cb = props.onchange.clone();
				Callback::from(move |credentials: onlivfe::vrchat::LoginRequestPart| {
					cb.emit(LoginCredentials::VRChat(Box::new(credentials)));
				})
			};

			html! {<VrcAccountCredentialsInput value={(**credentials).clone()} onchange={cb} />}
		}
		LoginCredentials::ChilloutVR(credentials) => {
			let cb = {
				let cb = props.onchange.clone();
				Callback::from(
					move |credentials: onlivfe::cvr::query::LoginCredentials| {
						cb.emit(LoginCredentials::ChilloutVR(Box::new(credentials)));
					},
				)
			};

			html! {<CvrAccountCredentialsInput value={(**credentials).clone()} onchange={cb} />}
		}
		LoginCredentials::NeosVR(credentials) => {
			let cb = {
				let cb = props.onchange.clone();
				Callback::from(
					move |credentials: onlivfe::neosvr::query::LoginCredentials| {
						cb.emit(LoginCredentials::NeosVR(Box::new(credentials)));
					},
				)
			};

			html! {<NeosAccountCredentialsInput value={(**credentials).clone()} onchange={cb} />}
		}
	}
}

#[function_component(VrcAccountCredentialsInput)]
fn vrc_account_credentials_input(
	props: &TwoWayBindingProps<onlivfe::vrchat::LoginRequestPart>,
) -> Html {
	use onlivfe::vrchat::LoginRequestPart;

	match &props.value {
		LoginRequestPart::LoginRequest(credentials) => {
			let on_username_change = {
				let cb = props.onchange.clone();
				let creds = credentials.clone();
				Callback::from(move |event: Event| {
					let input = event.target_unchecked_into::<HtmlInputElement>();
					let mut creds = creds.clone();
					creds.username = input.value();
					cb.emit(LoginRequestPart::LoginRequest(creds));
				})
			};

			let on_password_change = {
				let cb = props.onchange.clone();
				let creds = credentials.clone();
				Callback::from(move |event: Event| {
					let input = event.target_unchecked_into::<HtmlInputElement>();
					let mut creds = creds.clone();
					creds.password = input.value();
					cb.emit(LoginRequestPart::LoginRequest(creds));
				})
			};

			html! {
				<>
					<input required=true type="text" placeholder="Username" value={credentials.username.clone()} onchange={on_username_change} />
					<input required=true type="password" placeholder="Password" value={credentials.password.clone()} onchange={on_password_change} />
				</>
			}
		}
		LoginRequestPart::SecondFactor((id, second_factor)) => {
			use onlivfe::vrchat::query::VerifySecondFactor;
			match second_factor {
				VerifySecondFactor::Code(totp) => {
					let on_totp_change = {
						let cb = props.onchange.clone();
						let id = id.clone();
						Callback::from(move |event: Event| {
							let input = event.target_unchecked_into::<HtmlInputElement>();
							cb.emit(LoginRequestPart::SecondFactor((id.clone(), VerifySecondFactor::Code(input.value()))));
						})
					};

					html! {
						<>
							<input required=true type="number" placeholder="2FA" value={totp.clone()} onchange={on_totp_change} />
						</>
					}
				},
				VerifySecondFactor::Email(email_otp) => {
					let on_totp_change = {
						let cb = props.onchange.clone();
						let id = id.clone();
						Callback::from(move |event: Event| {
							let input = event.target_unchecked_into::<HtmlInputElement>();
							cb.emit(LoginRequestPart::SecondFactor((id.clone(), VerifySecondFactor::Email(input.value()))));
						})
					};

					html! {
						<>
							<input required=true type="number" placeholder="Email code" value={email_otp.clone()} onchange={on_totp_change} />
						</>
					}
				},
				VerifySecondFactor::Recovery(recovery) => {
					let on_totp_change = {
						let cb = props.onchange.clone();
						let id = id.clone();
						Callback::from(move |event: Event| {
							let input = event.target_unchecked_into::<HtmlInputElement>();
							cb.emit(LoginRequestPart::SecondFactor((id.clone(), VerifySecondFactor::Recovery(input.value()))));
						})
					};

					html! {
						<>
							<input required=true type="number" placeholder="Recovery code" value={recovery.clone()} onchange={on_totp_change} />
						</>
					}
				}
			}
		}
	}
}

#[function_component(CvrAccountCredentialsInput)]
fn cvr_account_credentials_input(
	props: &TwoWayBindingProps<onlivfe::cvr::query::LoginCredentials>,
) -> Html {
	let on_email_change = {
		let cb = props.onchange.clone();
		let creds = props.value.clone();
		Callback::from(move |event: Event| {
			let input = event.target_unchecked_into::<HtmlInputElement>();
			let mut creds = creds.clone();
			creds.email = input.value();
			cb.emit(creds);
		})
	};

	let on_password_change = {
		let cb = props.onchange.clone();
		let creds = props.value.clone();
		Callback::from(move |event: Event| {
			let input = event.target_unchecked_into::<HtmlInputElement>();
			let mut creds = creds.clone();
			creds.password = input.value();
			cb.emit(creds);
		})
	};

	html! {
		<>
			<input required=true type="email" placeholder="Email" value={props.value.email.clone()} onchange={on_email_change} />
			<input required=true type="password" placeholder="Password" value={props.value.password.clone()} onchange={on_password_change} />
		</>
	}
}

#[function_component(NeosAccountCredentialsInput)]
fn neos_account_credentials_input(
	props: &TwoWayBindingProps<onlivfe::neosvr::query::LoginCredentials>,
) -> Html {
	use onlivfe::neosvr::query::LoginCredentialsIdentifier;

	let identifier_picker = {
		let on_change = {
			let cb = props.onchange.clone();
			let creds = props.value.clone();
			Callback::from(move |event: Event| {
				let select = event.target_unchecked_into::<HtmlSelectElement>();
				let mut creds = creds.clone();
				creds.identifier = match select.value().as_str() {
					"Username" => LoginCredentialsIdentifier::Username(
						creds.identifier.inner().to_string(),
					),
					"Email" => LoginCredentialsIdentifier::Email(
						creds.identifier.inner().to_string(),
					),
					_ => LoginCredentialsIdentifier::OwnerID(
						creds.identifier.inner().to_string(),
					),
				};

				cb.emit(creds);
			})
		};
		html! {
			<select onchange={on_change}>
				<option value="Username">{"Username"}</option>
				<option value="Email">{"Email"}</option>
				<option value="OwnerId">{"User ID"}</option>
			</select>
		}
	};

	let identifier_html = match &props.value.identifier {
		LoginCredentialsIdentifier::OwnerID(user_id) => {
			let on_change = {
				let cb = props.onchange.clone();
				let creds = props.value.clone();
				Callback::from(move |event: Event| {
					let input = event.target_unchecked_into::<HtmlInputElement>();
					let mut creds = creds.clone();
					creds.identifier = LoginCredentialsIdentifier::OwnerID(input.value());
					cb.emit(creds);
				})
			};
			html! {
				<input required=true type="text" placeholder="User-ID" value={user_id.clone()} onchange={on_change} />
			}
		}
		LoginCredentialsIdentifier::Email(email) => {
			let on_change = {
				let cb = props.onchange.clone();
				let creds = props.value.clone();
				Callback::from(move |event: Event| {
					let input = event.target_unchecked_into::<HtmlInputElement>();
					let mut creds = creds.clone();
					creds.identifier = LoginCredentialsIdentifier::Email(input.value());
					cb.emit(creds);
				})
			};
			html! {
				<input required=true type="text" placeholder="Email" value={email.clone()} onchange={on_change} />
			}
		}
		LoginCredentialsIdentifier::Username(username) => {
			let on_change = {
				let cb = props.onchange.clone();
				let creds = props.value.clone();
				Callback::from(move |event: Event| {
					let input = event.target_unchecked_into::<HtmlInputElement>();
					let mut creds = creds.clone();
					creds.identifier =
						LoginCredentialsIdentifier::Username(input.value());
					cb.emit(creds);
				})
			};
			html! {
				<input required=true type="text" placeholder="Username" value={username.clone()} onchange={on_change} />
			}
		}
	};

	let on_password_change = {
		let cb = props.onchange.clone();
		let creds = props.value.clone();
		Callback::from(move |event: Event| {
			let input = event.target_unchecked_into::<HtmlInputElement>();
			let mut creds = creds.clone();
			creds.password = input.value();
			cb.emit(creds);
		})
	};

	let on_totp_change = {
		let cb = props.onchange.clone();
		let creds = props.value.clone();
		Callback::from(move |event: Event| {
			let input = event.target_unchecked_into::<HtmlInputElement>();
			let mut creds = creds.clone();
			let val = input.value();
			creds.totp = if val.is_empty() { None } else { Some(val) };
			cb.emit(creds);
		})
	};

	html! {
		<>
			<ul style="flex-direction: row; flex-wrap: wrap;">
				<li style="flex-grow: 1;">{identifier_picker}</li>
				<li style="flex-grow: 5;">{identifier_html}</li>
			</ul>
			<ul style="flex-direction: row; flex-wrap: wrap;">
				<li style="flex-grow: 8;"><input required=true type="password" placeholder="Password" value={props.value.password.clone()} onchange={on_password_change} /></li>
				<li style="flex-grow: 1;"><input required=false type="number" placeholder="2FA" value={props.value.totp.clone()} onchange={on_totp_change} /></li>
			</ul>
		</>
	}
}
