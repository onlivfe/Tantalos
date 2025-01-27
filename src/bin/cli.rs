#![cfg(feature = "cli")]

use clap::Parser;

#[derive(Parser)]
#[command(version)]
#[command(propagate_version = true)]
struct Cli {
	#[cfg(feature = "gui")]
	#[cfg_attr(
		feature = "gui",
		doc = "Starts the GUI, ignoring most other options"
	)]
	#[cfg_attr(feature = "gui", arg(short, long))]
	gui: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let cli = Cli::parse();

	#[cfg(feature = "gui")]
	if cli.gui {
		return tantalos::start_gui();
	}

	Ok(())
 }
