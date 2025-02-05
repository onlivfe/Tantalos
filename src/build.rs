fn main() {
	println!("cargo::rerun-if-changed=res/i18n/");

	let translations = std::fs::read_to_string("res/i18n/en-US.ftl").unwrap();
	#[cfg(debug_assertions)]
	let translations = translations.replace("=", "=🌐");
	std::fs::write("res/i18n/en-Zyyy.ftl", translations).unwrap();
}
