#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]
#![allow(non_snake_case)]

use dioxus_i18n::unic_langid::{LanguageIdentifier, langid as langId};
use dioxus_i18n::{prelude::*, t};
use freya::prelude::*;

static FIRA_SANS_FONT: &[u8] =
	include_bytes!("../../res/fonts/FiraSans-Regular.ttf");
static MATERIAL_ICONS_FONT: &[u8] =
	include_bytes!("../../res/fonts/material-icons.ttf");

//static APP_ICON: &[u8] = include_bytes!("../../res/icons/icon.png");

fn main() -> Result<(), Box<dyn std::error::Error>> {
	onlivfe_wrapper::init("Tantalos", env!("CARGO_PKG_VERSION")).unwrap();

	let config: LaunchConfigBuilder<'_, ()> = LaunchConfigBuilder::default();
	let config = config
		.without_default_fonts()
		.with_font("Fira Sans", FIRA_SANS_FONT)
		.with_font("Material Icons", MATERIAL_ICONS_FONT)
		//.with_icon(Icon::from_rgba(APP_ICON.to_owned(), 512, 512))
		.with_title("Tantalos")
		.with_default_font("Fira Sans")
		.with_background("black");
	let config = config.build();
	launch_cfg(app, config);

	Ok(())
}

//const LANGUAGES: &[LanguageIdentifier] = &[langId!("en-US"),
// langId!("fi-FI")];
fn i18n_config() -> (Vec<LanguageIdentifier>, I18nConfig) {
	//#[cfg(not(debug_assertions))]
	macro_rules! lang {
		($lang_id:literal) => {
			(
				langId!($lang_id),
				Locale::new_static(
					langId!($lang_id),
					include_str!(concat!("../../res/i18n/", $lang_id, ".ftl")),
				),
			)
		};
	}
	// Seems broken currently :/
	/*#[cfg(debug_assertions)]
	macro_rules! lang {
		($lang_id:literal) => {
			Locale::new_dynamic(
				id!($lang_id),
				concat!("../../res/i18n/", $lang_id, ".ftl"),
			)
		};
	}*/

	let langs_tuple = vec![lang!("en-US"), lang!("fi-FI")];

	let mut i18n = I18nConfig::new(langId!("fi-FI"));

	let mut lang_ids = Vec::with_capacity(langs_tuple.len());
	for (lang_id, locale) in langs_tuple {
		i18n = i18n.with_locale(locale);
		lang_ids.push(lang_id);
	}

	(lang_ids, i18n)
}

#[derive(PartialEq, Clone, Debug)]
struct I18nConf {
	pub languages: Vec<LanguageIdentifier>,
}

fn app() -> Element {
	let (languages, i18n) = i18n_config();
	use_init_i18n(|| i18n);
	use_context_provider(|| I18nConf { languages });

	const CUSTOM_THEME: Theme = Theme { ..DARK_THEME };

	rsx!(ThemeProvider {
					theme: CUSTOM_THEME,
					rect {
					height: "100%",
					width: "100%",
					content: "fit",
					main_align: "space-between",
						Header {},
						Main {},
						Footer {},
	}
	})
}

pub fn Header() -> Element {
	rsx!(rect {
		content: "flex",
		background: "rgb(0, 255, 0)",
		width: "fill-min"
	})
}
pub fn Footer() -> Element {
	rsx!(
		rect {
			background: "rgb(255, 0, 0)",
			width: "100%",
			LanguagePicker {}
		}
	)
}

fn Main() -> Element {
	let i18n = i18n();
	let mut count = use_signal(|| 0);

	rsx!(
		rect {
					width: "fill-min",
					main_align: "center",
					cross_align: "center",
					background: "rgb(0, 119, 182)",

								rect {

					shadow: "0 4 20 5 rgb(0, 0, 0, 80)",
					label {
							font_size: "75",
							font_weight: "bold",
							"{count}"
					}
			}
			rect {
					direction: "horizontal",
					Button {
							onclick: move |_| count += 1,
							label { { t!("increase") } }
					}
					Button {
							onclick: move |_| count -= 1,
							label { { t!("decrease") } }
					}
			}
		}

	)
}

fn LanguagePicker() -> Element {
	let mut i18n = i18n();
	let i18n_config = use_context::<I18nConf>();

	let languages: Vec<(LanguageIdentifier, String)> =
		i18n_config.languages.iter().map(|l| (l.clone(), format!("{l}"))).collect();

	rsx!(
		rect {
			for (lang_id, as_str) in languages.clone() {

					Button {
						label { { t!("language", lang: &as_str) } }
						onclick: move |_| i18n.set_language(lang_id.clone())
				}
				{ { as_str } }
			}
		}
	)
}

#[component]
fn icon(name: String) -> Element {
	rsx!(text { font_family: "Material Icons", { name } })
}
