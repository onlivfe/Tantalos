use tracing::error;

slint::include_modules!();

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
	#[cfg(debug_assertions)]
	slint::init_translations!(concat!(env!("CARGO_MANIFEST_DIR"), "/res/lang/"));

	let ui = MainWindow::new()?;

	#[cfg(not(debug_assertions))]
	if let Err(e) = slint::select_bundled_translation("en") {
		error!("Failed to select bundled translation: {e}");
	}

	if let Err(e) = slint::set_xdg_app_id("com.munally.tantalos") {
		error!("Failed to set XDG app ID: {e}");
	}

	ui.run()?;

	Ok(())
}
