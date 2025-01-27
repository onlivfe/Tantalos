// Prevent console window in addition to Slint window in Windows release builds
// when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate tracing;

#[cfg(feature = "gui")]
pub mod slint;

fn setup() {
	#[cfg(debug_assertions)]
	dotenvy::dotenv().expect("Dotenv loading to work");

	#[cfg(debug_assertions)]
	let format = tracing_subscriber::fmt::format().pretty();
	#[cfg(not(debug_assertions))]
	let format = tracing_subscriber::fmt::format().json();

	tracing_subscriber::fmt()
		.event_format(format)
		.with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
		.init();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
#[cfg(feature = "gui")]
pub fn start_gui() -> Result<(), Box<dyn std::error::Error>> {
	setup();
	slint::run()
}
 
