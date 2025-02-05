#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]
#![allow(non_snake_case)]

use std::borrow::Cow;

use components::{I18nConf, Layout, LayoutConfig, i18n_config};
use dioxus::prelude::*;
use dioxus_i18n::prelude::use_init_i18n;
use tracing::{info, warn};
use views::{Accounts, Dashboard, Info, Settings};

mod components;
mod views;

#[macro_export]
macro_rules! tid {
	($id:expr, $( $name:ident : $value:expr ),* ) => {
			dioxus_i18n::te!($id, $( $name : $value ),*).unwrap_or_else(|_|"␂".to_owned() + $id)
	};

	($id:expr ) => {{
			dioxus_i18n::te!($id).unwrap_or_else(|_|"␂".to_owned() + $id)
	}};
}

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
	#[layout(Layout)]
	#[redirect("/", || Route::Info)]
	#[route("/info")]
	Info,
	#[route("/dashboard")]
	Dashboard,
	#[route("/accounts")]
	Accounts,
	#[route("/settings")]
	Settings,
}

#[cfg(feature = "server")]
pub type Onlivfe = std::sync::Arc<onlivfe_wrapper::Onlivfe<onlivfe_wrapper::onlivfe_cache_store::OnlivfeCacheStorageBackend>>;

const FAVICON: Asset = asset!("/res/icons/favicon.ico");
const MAIN_CSS: Asset = asset!("/res/css/main.css");

const APP_NAME: &str = "Tantalos";

fn main() {
	#[cfg(feature = "server")]
	onlivfe_wrapper::init(APP_NAME, env!("CARGO_PKG_VERSION")).unwrap();

	let config = dioxus::LaunchBuilder::new();
	
	#[cfg(feature = "server")]
	let config = config.with_context_provider(
		|| Box::new(Onlivfe::new(onlivfe_wrapper::Onlivfe::new(
			onlivfe_wrapper::onlivfe_cache_store::OnlivfeCacheStorageBackend::new(APP_NAME).expect("Creating backend cache store to succeed")
		).expect("Creating backend Onlivfe to succeed")))
	);
	
	#[cfg(feature = "web")]
	let config = config.with_cfg(dioxus::web::Config::new().hydrate(true));

	#[cfg(feature = "server")]
	info!("Server started");

	config.launch(App);
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
	
	let _auth_ok = use_resource(ensure_authenticated);

	document::eval(
		r#"document.documentElement.setAttribute('data-theme', 'dark')"#,
	);
	rsx! {
		document::Style { {"html { background: black; }"} }
		// Global app resources
		document::Link { rel: "icon", href: FAVICON }
		document::Link { rel: "stylesheet", href: MAIN_CSS }
		Router::<Route> {}
	}
}


#[server(EnsureAuthenticated)]
async fn ensure_authenticated() -> Result<(), ServerFnError> {
	let FromContext(onlivfe): FromContext<crate::Onlivfe> = extract().await?;
	let res = onlivfe.re_authenticate(false).await;
	if let Err(err) = &res {
		tracing::warn!("Reauthenticating failed for {err:?}");
	}
	Ok(())
}
