//! The tauri wrapper app for the onlivfe web UI

#![cfg_attr(nightly, feature(doc_auto_cfg))]
#![deny(clippy::all)]
#![forbid(unsafe_code)]
#![deny(clippy::cargo)]
#![deny(rustdoc::invalid_html_tags)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
// Not much can be done about it :/
#![allow(clippy::multiple_crate_versions)]
// Better to be able to name types properly for TS
#![allow(clippy::module_name_repetitions)]
// We're using the same that other onlivfe crates do
#![allow(clippy::wildcard_dependencies)]
#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use onlivfe::{LoginError, PlatformAccount, PlatformAccountId};

type Interface =
	onlivfe_wrapper::Onlivfe<onlivfe_cache_store::OnlivfeCacheStorageBackend>;

fn main() {
	onlivfe_wrapper::init("desktop", env!("CARGO_PKG_VERSION")).unwrap();
	let store =
		onlivfe_cache_store::OnlivfeCacheStorageBackend::new("VRPeeps").unwrap();
	let interface: Interface = onlivfe_wrapper::Onlivfe::new(store).unwrap();
	tauri::Builder::default()
		.manage(interface)
		.invoke_handler(tauri::generate_handler![
			authenticated_accounts,
			login,
			save_and_exit
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}

#[tauri::command]
async fn authenticated_accounts(
	interface: tauri::State<'_, Interface>,
) -> Result<Vec<PlatformAccount>, String> {
	let account_ids = interface.authenticated_accounts().await?;
	// TODO: proper async loop
	let mut accounts = vec![];
	for account_id in account_ids {
		accounts
			.push(interface.platform_account(account_id.clone(), account_id).await?);
	}

	Ok(accounts)
}

#[tauri::command]
async fn login(
	credentials: onlivfe::LoginCredentials,
	interface: tauri::State<'_, Interface>,
) -> Result<Result<PlatformAccountId, LoginError>, ()> {
	Ok(interface.login(credentials).await)
}

#[tauri::command]
fn save_and_exit(window: tauri::Window) -> Result<(), String> {
	// TODO: Saving?
	window.close().map_err(|e| e.to_string())?;
	Ok(())
}
