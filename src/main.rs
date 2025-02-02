#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]
#![allow(non_snake_case)]

use std::borrow::Cow;

use components::{I18nConf, Layout, LayoutConfig, i18n_config};
use dioxus::prelude::*;
use dioxus_i18n::prelude::use_init_i18n;
use views::{Accounts, Info, Settings};

mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
	#[layout(Layout)]
	#[redirect("/", || Route::Info)]
	#[route("/info")]
	Info,
	#[route("/accounts")]
	Accounts,
	#[route("/settings")]
	Settings,
}

const FAVICON: Asset = asset!("/res/icons/favicon.ico");
const MAIN_CSS: Asset = asset!("/res/css/main.css");

fn main() {
	#[cfg(not(target_arch = "wasm32"))]
	onlivfe_wrapper::init("Tantalos", env!("CARGO_PKG_VERSION")).unwrap();

	#[cfg(all(not(feature = "web"), not(feature = "mobile")))]
	let config = ();
	#[cfg(all(not(feature = "web"), feature = "mobile"))]
	let config = dioxus::mobile::Config::new();
	#[cfg(feature = "web")]
	let config = { dioxus::web::Config::new().hydrate(true) };

	dioxus::LaunchBuilder::new().with_cfg(config).launch(App);
}

#[derive(Clone, Debug, PartialEq)]
pub struct ColorScheme {
	pub primary: Cow<'static, str>,
	pub background: Cow<'static, str>,
	pub secondary: Cow<'static, str>,
}

impl ColorScheme {
	const DEFAULT_BG: &'static str = "#FF2F6E";
	const DEFAULT_PRIMARY: &'static str = "#00FFE8";
	const DEFAULT_SECONDARY: &'static str = "#111111";
}

impl Default for ColorScheme {
	fn default() -> Self {
		Self {
			primary: ColorScheme::DEFAULT_PRIMARY.into(),
			background: ColorScheme::DEFAULT_BG.into(),
			secondary: ColorScheme::DEFAULT_SECONDARY.into(),
		}
	}
}

#[component]
fn App() -> Element {
	let (languages, i18n) = i18n_config();
	use_init_i18n(|| i18n);
	use_context_provider(|| I18nConf { languages });
	use_context_provider(ColorScheme::default);
	use_context_provider(|| Signal::new(LayoutConfig::default()));

	document::eval(
		r#"document.documentElement.setAttribute('data-theme', 'dark')"#,
	);
	rsx! {
		// Global app resources
		document::Link { rel: "icon", href: FAVICON }
		document::Link { rel: "stylesheet", href: MAIN_CSS }
		Router::<Route> {}
	}
}
