use std::sync::Arc;

use eframe::egui::{SelectableLabel, TextEdit, Ui};
use onlivfe::{LoginCredentials, PlatformType};
use strum::IntoEnumIterator;

use crate::{HistoryBehavior, UpdatablePage};

const NEW_VRC_LOGIN_REQ: onlivfe::vrchat::LoginRequestPart =
	onlivfe::vrchat::LoginRequestPart::LoginRequest(
		onlivfe::vrchat::query::Authenticating {
			username: String::new(),
			password: String::new(),
		},
	);

#[derive(Clone)]
pub struct Page<Store: onlivfe::storage::OnlivfeStore + 'static> {
	i: Arc<onlivfe_wrapper::Onlivfe<Store>>,
	credentials: onlivfe::LoginCredentials,
}
impl<Store: onlivfe::storage::OnlivfeStore + 'static> From<Page<Store>>
	for crate::Page<Store>
{
	fn from(value: Page<Store>) -> Self { Self::AddAccount(value) }
}

impl<Store: onlivfe::storage::OnlivfeStore + 'static> Page<Store> {
	#[must_use]
	pub fn new(i: Arc<onlivfe_wrapper::Onlivfe<Store>>) -> Self {
		Self { i, credentials: onlivfe::LoginCredentials::VRChat(Box::new(NEW_VRC_LOGIN_REQ)) }
	}
}

impl<Store: onlivfe::storage::OnlivfeStore + 'static> UpdatablePage<Store>
	for Page<Store>
{
	fn update(
		&mut self, ui: &mut Ui, _ctx: &eframe::egui::Context,
		_i: Arc<onlivfe_wrapper::Onlivfe<Store>>,
	) -> Option<(crate::Page<Store>, HistoryBehavior)> {
		ui.heading("Add account");

		ui.horizontal_wrapped(|ui| {
			ui.label("Platform:");
			for platform_type in PlatformType::iter() {
				if ui
					.add(eframe::egui::SelectableLabel::new(
						self.credentials.platform() == platform_type,
						platform_type.as_ref(),
					))
					.clicked()
				{
					self.credentials = match platform_type {
						PlatformType::VRChat => onlivfe::LoginCredentials::VRChat(
							Box::new(NEW_VRC_LOGIN_REQ.clone()),
						),
						PlatformType::ChilloutVR => onlivfe::LoginCredentials::ChilloutVR(
							Box::new(onlivfe::cvr::query::LoginCredentials {
								email: String::new(),
								password: String::new(),
							}),
						),
						PlatformType::NeosVR => onlivfe::LoginCredentials::NeosVR(
							Box::new(onlivfe::neosvr::query::LoginCredentials::new(
								onlivfe::neosvr::query::LoginCredentialsIdentifier::Username(
									String::new(),
								),
								"",
							)),
						),
					}
				}
			}
		});

		match &mut self.credentials {
			LoginCredentials::VRChat(login_req_part) => match &mut **login_req_part {
				onlivfe::vrchat::LoginRequestPart::LoginRequest(creds) => {
					ui.add(
						TextEdit::singleline(&mut creds.username).hint_text("Username"),
					);
					ui.add(
						TextEdit::singleline(&mut creds.password)
							.password(true)
							.hint_text("Password"),
					);
				}
				onlivfe::vrchat::LoginRequestPart::SecondFactor(second_factor) => {
					match second_factor {
						onlivfe::vrchat::query::VerifySecondFactor::Code(totp) => {
							ui.add(TextEdit::singleline(totp).hint_text("TOTP"));
							if ui.button("Use recovery code instead").clicked() {
								let mut recovery =
									onlivfe::vrchat::query::VerifySecondFactor::Recovery(
										String::new(),
									);
								std::mem::swap(second_factor, &mut recovery);
							}
						}
						onlivfe::vrchat::query::VerifySecondFactor::Email(email) => {
							ui.add(TextEdit::singleline(email).hint_text("Email code"));
						}
						onlivfe::vrchat::query::VerifySecondFactor::Recovery(recovery) => {
							ui.add(TextEdit::singleline(recovery).hint_text("Recovery code"));
						}
					}
				}
			},
			LoginCredentials::ChilloutVR(creds) => {
				ui.add(TextEdit::singleline(&mut creds.email).hint_text("Email"));
				ui.add(
					TextEdit::singleline(&mut creds.password)
						.password(true)
						.hint_text("Password"),
				);
			}
			LoginCredentials::NeosVR(creds) => {
				neos_identifier_picker(&mut creds.identifier, ui, false);
				ui.add(
					TextEdit::singleline(&mut creds.password)
						.password(true)
						.hint_text("Password"),
				);
			}
		}

		None
	}
}

fn neos_identifier_picker(
	identifier: &mut onlivfe::neosvr::query::LoginCredentialsIdentifier,
	ui: &mut Ui, is_loading: bool,
) {
	eframe::egui::ComboBox::from_label("Login type")
		.selected_text(identifier.as_ref())
		.show_ui(ui, |ui| {
			if ui
				.add(SelectableLabel::new(
					matches!(
						identifier,
						onlivfe::neosvr::query::LoginCredentialsIdentifier::Username(_)
					),
					"Username",
				))
				.clicked()
			{
				let mut new_identifier =
					onlivfe::neosvr::query::LoginCredentialsIdentifier::Username(
						identifier.inner().into(),
					);
				std::mem::swap(identifier, &mut new_identifier);
			}

			if ui
				.add(SelectableLabel::new(
					matches!(
						identifier,
						onlivfe::neosvr::query::LoginCredentialsIdentifier::Email(_)
					),
					"Email",
				))
				.clicked()
			{
				let mut new_identifier =
					onlivfe::neosvr::query::LoginCredentialsIdentifier::Email(
						identifier.inner().into(),
					);
				std::mem::swap(identifier, &mut new_identifier);
			}

			if ui
				.add(SelectableLabel::new(
					matches!(
						identifier,
						onlivfe::neosvr::query::LoginCredentialsIdentifier::OwnerID(_)
					),
					"OwnerID",
				))
				.clicked()
			{
				let mut new_identifier =
					onlivfe::neosvr::query::LoginCredentialsIdentifier::OwnerID(
						identifier.inner().into(),
					);
				std::mem::swap(identifier, &mut new_identifier);
			}
		});

	let label = identifier.as_ref().to_string();

	ui.add(
		TextEdit::singleline(identifier.inner_mut())
			.hint_text(label)
			.interactive(!is_loading),
	);
}
