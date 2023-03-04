use eframe::{
	egui::{Context, FontData, FontDefinitions, TextStyle},
	epaint::FontFamily,
};

/// Sets up the fonts
///
/// # Panics
///
/// If an internal error happens
pub fn setup(ctx: &Context) {
	let mut style = (*ctx.style()).clone();

	let mut fonts = FontDefinitions::default();

	if let Some(font_id) = style.text_styles.get_mut(&TextStyle::Heading) {
		font_id.size = 34_f32;
	}

	if let Some(font_id) = style.text_styles.get_mut(&TextStyle::Body) {
		font_id.size = 24_f32;
	}
	if let Some(font_id) = style.text_styles.get_mut(&TextStyle::Button) {
		font_id.size = 26_f32;
	}

	if let Some(font_id) = style.text_styles.get_mut(&TextStyle::Monospace) {
		font_id.size = 22_f32;
	}

	if let Some(font_id) = style.text_styles.get_mut(&TextStyle::Small) {
		font_id.size = 16_f32;
	}

	fonts.font_data.insert(
		"raleway".to_owned(),
		FontData::from_static(include_bytes!("../res/Raleway.ttf")),
	);
	fonts
		.families
		.get_mut(&FontFamily::Proportional)
		.unwrap()
		.insert(0, "raleway".to_owned());
	fonts
		.families
		.get_mut(&FontFamily::Monospace)
		.unwrap()
		.push("raleway".to_owned());

	#[cfg(feature = "font-kit")]
	{
		const JP_FONT_ERR: &str = "This might cause some characters, like japanese ones, to not render properly.";
		const JP_FONT: &str = "Noto Sans CJK JP font";

		use font_kit::family_name::FamilyName;
		use font_kit::properties::{Properties, Style, Weight};
		use font_kit::source::SystemSource;
		if let Ok(font_handle) = SystemSource::new().select_best_match(
			&[FamilyName::Title("Noto Sans CJK JP".to_string())],
			Properties::new().style(Style::Normal).weight(Weight::LIGHT),
		) {
			if let Ok(font) = font_handle.load() {
				if let Some(font_data) = font.copy_font_data() {
					fonts.font_data.insert(
						"noto-cjk-jp".to_owned(),
						FontData::from_owned((*font_data).clone()),
					);
					fonts
						.families
						.get_mut(&FontFamily::Proportional)
						.unwrap()
						.push("noto-cjk-jp".to_owned());
					fonts
						.families
						.get_mut(&FontFamily::Monospace)
						.unwrap()
						.push("noto-cjk-jp".to_owned());
				} else {
					eprintln!("Failed to load the data of {JP_FONT}. {JP_FONT_ERR}");
				}
			} else {
				eprintln!("Failed to load {JP_FONT}. {JP_FONT_ERR}");
			}
		} else {
			eprintln!("Couldn't find {JP_FONT}. {JP_FONT_ERR}");
		}
	}

	ctx.set_fonts(fonts);
}
