//! FluentBundle management and locale loading

use fluent_bundle::bundle::FluentBundle;
use fluent_bundle::FluentResource;
use intl_memoizer::concurrent::IntlLangMemoizer;
use unic_langid::LanguageIdentifier;

/// Thread-safe FluentBundle type (uses concurrent IntlLangMemoizer).
/// This is required because Crux stores Model (which contains Localiser)
/// inside a LazyLock<Bridge> which requires Send + Sync.
pub type ConcurrentFluentBundle = FluentBundle<FluentResource, IntlLangMemoizer>;

/// All supported locales with their display names
pub const SUPPORTED_LOCALES: &[(&str, &str)] = &[
    ("en", "English"),
    // Add new locales here as translations are added:
    // ("fr", "Français"),
    // ("de", "Deutsch"),
    // ("es", "Español"),
    // ("ja", "日本語"),
];

/// Embedded English .ftl files (compile-time inclusion — no filesystem access needed)
mod en {
    pub const COMMON: &str = include_str!("../locales/en/common.ftl");
    pub const DEMO: &str = include_str!("../locales/en/demo.ftl");
    pub const PLAYBACK: &str = include_str!("../locales/en/playback.ftl");
    pub const PLAYBACK_INFO: &str = include_str!("../locales/en/playback-info.ftl");
    pub const SUBTITLE_STYLES: &str = include_str!("../locales/en/subtitle-styles.ftl");
    pub const JELLYFIN: &str = include_str!("../locales/en/jellyfin.ftl");
    pub const ACCESSIBILITY: &str = include_str!("../locales/en/accessibility.ftl");
}

/// Returns all .ftl source strings for the given locale code.
/// Falls back to English if the locale is not supported.
fn ftl_sources_for_locale(locale: &str) -> &'static [&'static str] {
    match locale {
        "en" => &[
            en::COMMON,
            en::DEMO,
            en::PLAYBACK,
            en::PLAYBACK_INFO,
            en::SUBTITLE_STYLES,
            en::JELLYFIN,
            en::ACCESSIBILITY,
        ],
        // Add new locale arms here:
        // "fr" => &[fr::COMMON, fr::DEMO, ...],
        _ => ftl_sources_for_locale("en"), // fallback
    }
}

/// Build a thread-safe FluentBundle for the given locale, loading all .ftl resources.
pub fn build_bundle(locale: &str) -> ConcurrentFluentBundle {
    let lang_id: LanguageIdentifier = locale
        .parse()
        .unwrap_or_else(|_| "en".parse().expect("en is valid"));

    let mut bundle = ConcurrentFluentBundle::new_concurrent(vec![lang_id]);

    // Disable Unicode isolation marks (they cause invisible characters in output)
    bundle.set_use_isolating(false);

    for source in ftl_sources_for_locale(locale) {
        let resource = FluentResource::try_new(source.to_string())
            .expect("Failed to parse FTL resource");
        bundle
            .add_resource(resource)
            .expect("Failed to add FTL resource to bundle");
    }

    bundle
}
