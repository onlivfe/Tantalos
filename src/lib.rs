//! Fully rust powered [onlivfe](https://onlivfe.com) app.
//!
//! Very WIP alternative to compete against the [web based desktop app](https://github.com/onlivfe/desktop).

#![cfg_attr(nightly, feature(doc_auto_cfg))]
#![deny(clippy::all)]
#![forbid(unsafe_code)]
#![deny(clippy::cargo)]
#![deny(rustdoc::invalid_html_tags)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
// My project my choice, tabs are literally made for indentation, spaces not.
#![allow(clippy::tabs_in_doc_comments)]
// Not much can be done about it :/
#![allow(clippy::multiple_crate_versions)]

pub mod about;
pub mod add_account;
pub mod app;
pub mod dash;
pub mod fonts;
pub mod settings;

/// Starts the application
///
/// # Errors
///
/// If the app encountered an error whilst starting or running
pub fn start() -> Result<(), Box<dyn std::error::Error>> {
	let store = onlivfe_cache_store::OnlivfeCacheStorageBackend::new("app_rs")?;
	let interface = onlivfe_wrapper::Onlivfe::new(store)?;

	let native_options = eframe::NativeOptions::default();
	let app_creator: eframe::AppCreator = Box::new(move |creation_ctx| {
		Box::new(app::Onlivfe::new(creation_ctx, interface))
	});
	eframe::run_native(env!("CARGO_PKG_NAME"), native_options, app_creator)
		.expect("starting the app");

	Ok(())
}
