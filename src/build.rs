use std::process::{Command, ExitStatus};

fn main() {
	let mut config = slint_build::CompilerConfiguration::new()
		.with_style("material".to_string());

	println!("cargo::rerun-if-changed=res/lang");
	if cfg!(not(debug_assertions)) {
		println!("cargo:warning=Using embedded translations");
		config = config.with_bundled_translations("res/lang");
	} else {
		println!("cargo:warning=Using runtime translations");
	}

	let exit_status = Command::new("bash")
		.arg("res/lang/compile-translations.sh")
		.spawn()
		.expect("Spawning translation compilation to work")
		.wait()
		.expect("Compiling translations to work")
		.code();

	if !matches!(exit_status, Some(0i32)) {
		panic!("Translation compilation returned exit status: {:?}", exit_status)
	}

	slint_build::compile_with_config("res/ui/index.slint", config)
		.expect("Slint build failed");
}
