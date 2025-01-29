use clap::Parser;

#[derive(Parser)]
#[command(version)]
#[command(propagate_version = true)]
struct Cli {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	onlivfe_wrapper::init("tantalos-cli", env!("CARGO_PKG_VERSION")).unwrap();

	let cli = Cli::parse();

	Ok(())
}
