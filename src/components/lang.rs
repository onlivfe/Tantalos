use dioxus::prelude::*;
use dioxus_i18n::unic_langid::{LanguageIdentifier, langid as langId};
use dioxus_i18n::{prelude::*, tid};

use super::VerticalDirection;
use crate::components::Icon;

//const LANGUAGES: &[LanguageIdentifier] = &[langId!("en-US"),
// langId!("fi-FI")];
pub fn i18n_config() -> (Vec<LanguageIdentifier>, I18nConfig) {
	//#[cfg(not(debug_assertions))]
	macro_rules! lang {
		($lang_id:literal) => {
			(
				langId!($lang_id),
				LocaleResource::from(include_str!(concat!(
					"../../res/i18n/",
					$lang_id,
					".ftl"
				))),
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

	let mut i18n =
		I18nConfig::new(langId!("en-US")).with_fallback(langId!("en-US"));

	let mut lang_ids = Vec::with_capacity(langs_tuple.len());
	for (lang_id, locale_res) in langs_tuple {
		i18n = i18n.with_locale((lang_id.clone(), locale_res));
		lang_ids.push(lang_id);
	}

	(lang_ids, i18n)
}

#[derive(PartialEq, Clone, Debug)]
pub struct I18nConf {
	pub languages: Vec<LanguageIdentifier>,
}

#[component]
pub fn LanguagePicker(
	#[props(default = true)] compact: bool,
	#[props(default = VerticalDirection::Down)] open_direction: VerticalDirection,
) -> Element {
	let mut i18n = i18n();
	let i18n_config = use_context::<I18nConf>();

	let languages: Vec<(LanguageIdentifier, String)> =
		i18n_config.languages.iter().map(|l| (l.clone(), format!("{l}"))).collect();

	rsx! {
		details { class: "dropdown",
			summary { class: "outline", role: "button",
				span {
					Icon { name: "language" }
					if !compact {
						{" "}
						{tid!("language", selector : "true", lang : i18n.language().to_string())}
					}
				}
			}
			ul {
				position: if open_direction == VerticalDirection::Down { None } else { Some("absolute") },
				bottom: if open_direction == VerticalDirection::Down { None } else { Some("100%") },
				for (lang_id , as_str) in languages.clone() {
					li {
						a {
							aria_current: i18n.language() == lang_id,
							onclick: move |_| i18n.set_language(lang_id.clone()),
							{tid!("language", selector : "false", lang : & as_str)}
						}
					}
				}
			}
		}
	}
}
