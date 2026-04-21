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
    ("zh-CN", "简体中文"),
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

/// Embedded Simplified Chinese .ftl files
mod zh_cn {
    pub const COMMON: &str = include_str!("../locales/zh-CN/common.ftl");
    pub const DEMO: &str = include_str!("../locales/zh-CN/demo.ftl");
    pub const PLAYBACK: &str = include_str!("../locales/zh-CN/playback.ftl");
    pub const PLAYBACK_INFO: &str = include_str!("../locales/zh-CN/playback-info.ftl");
    pub const SUBTITLE_STYLES: &str = include_str!("../locales/zh-CN/subtitle-styles.ftl");
    pub const JELLYFIN: &str = include_str!("../locales/zh-CN/jellyfin.ftl");
    pub const ACCESSIBILITY: &str = include_str!("../locales/zh-CN/accessibility.ftl");
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
        "zh-CN" => &[
            zh_cn::COMMON,
            zh_cn::DEMO,
            zh_cn::PLAYBACK,
            zh_cn::PLAYBACK_INFO,
            zh_cn::SUBTITLE_STYLES,
            zh_cn::JELLYFIN,
            zh_cn::ACCESSIBILITY,
        ],
        // Add new locale arms here:
        // "fr" => &[fr::COMMON, fr::DEMO, ...],
        _ => ftl_sources_for_locale("en"), // fallback
    }
}

/// Negotiate the best supported locale for a device locale tag.
///
/// Tries in order:
/// 1. Exact match (e.g. "zh-CN" → "zh-CN")
/// 2. Language + region from BCP 47 (e.g. "zh-Hans-CN" → "zh-CN")
/// 3. Language-only match (e.g. "en-US" → "en", "zh" → "zh-CN")
/// 4. Falls back to "en"
pub fn negotiate_locale(requested: &str) -> &'static str {
    // 1. Exact match
    if let Some((code, _)) = SUPPORTED_LOCALES.iter().find(|(c, _)| *c == requested) {
        return code;
    }

    // Parse the BCP 47 tag
    if let Ok(lang_id) = requested.parse::<LanguageIdentifier>() {
        let lang = lang_id.language.as_str();
        let region = lang_id.region.as_ref().map(|r| r.as_str());

        // 2. Try language-region (handles "zh-Hans-CN" → "zh-CN")
        if let Some(region) = region {
            let lang_region = format!("{}-{}", lang, region);
            if let Some((code, _)) = SUPPORTED_LOCALES.iter().find(|(c, _)| *c == lang_region) {
                return code;
            }
        }

        // 3. Language-only: find first supported locale matching the base language
        if let Some((code, _)) = SUPPORTED_LOCALES.iter().find(|(c, _)| {
            c.parse::<LanguageIdentifier>()
                .map(|id| id.language.as_str() == lang)
                .unwrap_or(false)
        }) {
            return code;
        }
    }

    // 4. Fallback
    "en"
}

/// Build a thread-safe FluentBundle for the given locale, loading all .ftl resources.
/// The locale tag is negotiated against SUPPORTED_LOCALES first.
pub fn build_bundle(locale: &str) -> ConcurrentFluentBundle {
    let resolved = negotiate_locale(locale);

    let lang_id: LanguageIdentifier = resolved
        .parse()
        .unwrap_or_else(|_| "en".parse().expect("en is valid"));

    let mut bundle = ConcurrentFluentBundle::new_concurrent(vec![lang_id]);

    // Disable Unicode isolation marks (they cause invisible characters in output)
    bundle.set_use_isolating(false);

    for source in ftl_sources_for_locale(resolved) {
        let resource = FluentResource::try_new(source.to_string())
            .expect("Failed to parse FTL resource");
        bundle
            .add_resource(resource)
            .expect("Failed to add FTL resource to bundle");
    }

    bundle
}
