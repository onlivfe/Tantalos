fn main() {
	#[cfg(target = "windows")]
	{
		use std::env;
		if env::var("PROFILE").unwrap() == "release"
			&& env::var("CARGO_CFG_WINDOWS").is_ok()
		{
			let mut res = winres::WindowsResource::new();
			if let Ok(host) = env::var("HOST") {
				if host.contains("linux") {
					res
						.set_toolkit_path("/usr/bin")
						.set_windres_path("x86_64-w64-mingw32-windres");
				}
			}
			res.set_icon("static/logo.ico");
			res.compile().unwrap();
		}
	}
}
