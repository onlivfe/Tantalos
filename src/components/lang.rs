use dioxus::prelude::*;
use dioxus_i18n::unic_langid::{LanguageIdentifier, langid as langId};
use dioxus_i18n::{prelude::*, t};

//const LANGUAGES: &[LanguageIdentifier] = &[langId!("en-US"),
// langId!("fi-FI")];
pub fn i18n_config() -> (Vec<LanguageIdentifier>, I18nConfig) {
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
pub struct I18nConf {
	pub languages: Vec<LanguageIdentifier>,
}

pub fn LanguagePicker() -> Element {
	let mut i18n = i18n();
	let i18n_config = use_context::<I18nConf>();

	let languages: Vec<(LanguageIdentifier, String)> =
		i18n_config.languages.iter().map(|l| (l.clone(), format!("{l}"))).collect();

	rsx! {
		ul {
			for (lang_id, as_str) in languages.clone() {
				li {
					button {
							onclick: move |_| i18n.set_language(lang_id.clone()),
							span { {t!("language", lang: &as_str)} }
					}
				}
			}
	 }
	}
}
