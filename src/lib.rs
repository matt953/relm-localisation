//! relm-localisation — unified localisation crate for Relm Media
//!
//! Provides a `Localiser` that resolves Fluent translation keys for a given locale.
//! All .ftl translation files are embedded at compile time, so no filesystem access
//! is needed at runtime (works on iOS, Android, WASM, etc.).
//!
//! # Usage
//!
//! ```rust
//! use relm_localisation::Localiser;
//!
//! let l10n = Localiser::new("en");
//! assert_eq!(l10n.t("cancel"), "Cancel");
//!
//! let count_label = l10n.t_args("count-label", &[("count", 42.into())]);
//! assert_eq!(count_label, "Count: 42");
//! ```

mod bundle;

use fluent::{FluentArgs, FluentBundle, FluentResource, FluentValue};

pub use bundle::SUPPORTED_LOCALES;

/// Main localisation resolver. Holds a FluentBundle for the active locale
/// and resolves translation keys to strings.
pub struct Localiser {
    bundle: FluentBundle<FluentResource>,
    locale: String,
}

impl Localiser {
    /// Create a new Localiser for the given locale code (e.g., "en", "fr").
    /// Falls back to English if the locale is not supported.
    pub fn new(locale: &str) -> Self {
        Self {
            bundle: bundle::build_bundle(locale),
            locale: locale.to_string(),
        }
    }

    /// Change the active locale. Rebuilds the internal FluentBundle.
    pub fn set_locale(&mut self, locale: &str) {
        self.locale = locale.to_string();
        self.bundle = bundle::build_bundle(locale);
    }

    /// Get the current locale code.
    pub fn locale(&self) -> &str {
        &self.locale
    }

    /// Returns all supported locales as (code, display_name) pairs.
    pub fn available_locales() -> &'static [(&'static str, &'static str)] {
        SUPPORTED_LOCALES
    }

    /// Resolve a simple key with no arguments.
    ///
    /// Returns the key itself if not found (makes missing translations visible).
    pub fn t(&self, key: &str) -> String {
        self.resolve(key, None)
    }

    /// Resolve a key with named arguments for interpolation/plurals.
    ///
    /// # Example
    /// ```rust
    /// use relm_localisation::Localiser;
    ///
    /// let l10n = Localiser::new("en");
    /// let result = l10n.t_args("count-label", &[("count", 5.into())]);
    /// assert_eq!(result, "Count: 5");
    /// ```
    pub fn t_args(&self, key: &str, args: &[(&str, FluentValue<'_>)]) -> String {
        let mut fluent_args = FluentArgs::new();
        for (k, v) in args {
            fluent_args.set(*k, v.clone());
        }
        self.resolve(key, Some(&fluent_args))
    }

    fn resolve(&self, key: &str, args: Option<&FluentArgs<'_>>) -> String {
        let msg = match self.bundle.get_message(key) {
            Some(msg) => msg,
            None => return key.to_string(), // key not found — return raw key
        };

        let pattern = match msg.value() {
            Some(pattern) => pattern,
            None => return key.to_string(),
        };

        let mut errors = vec![];
        let result = self.bundle.format_pattern(pattern, args, &mut errors);

        // Log errors in debug builds but still return the best-effort result
        #[cfg(debug_assertions)]
        for err in &errors {
            eprintln!("Fluent error for key '{}': {}", key, err);
        }

        result.to_string()
    }
}

impl Default for Localiser {
    fn default() -> Self {
        Self::new("en")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_key() {
        let l10n = Localiser::new("en");
        assert_eq!(l10n.t("cancel"), "Cancel");
        assert_eq!(l10n.t("save"), "Save");
        assert_eq!(l10n.t("done"), "Done");
    }

    #[test]
    fn test_interpolation() {
        let l10n = Localiser::new("en");
        let result = l10n.t_args("count-label", &[("count", FluentValue::from(42))]);
        assert_eq!(result, "Count: 42");
    }

    #[test]
    fn test_missing_key_returns_key() {
        let l10n = Localiser::new("en");
        assert_eq!(l10n.t("nonexistent-key"), "nonexistent-key");
    }

    #[test]
    fn test_locale_switching() {
        let mut l10n = Localiser::new("en");
        assert_eq!(l10n.locale(), "en");

        // Switching to unsupported locale should fallback to English
        l10n.set_locale("zz");
        assert_eq!(l10n.locale(), "zz");
        assert_eq!(l10n.t("cancel"), "Cancel"); // English fallback
    }

    #[test]
    fn test_available_locales() {
        let locales = Localiser::available_locales();
        assert!(!locales.is_empty());
        assert!(locales.iter().any(|(code, _)| *code == "en"));
    }

    #[test]
    fn test_default() {
        let l10n = Localiser::default();
        assert_eq!(l10n.locale(), "en");
    }

    #[test]
    fn test_all_categories_loaded() {
        let l10n = Localiser::new("en");
        // Common
        assert_eq!(l10n.t("cancel"), "Cancel");
        // Demo
        assert_eq!(l10n.t("demo-title"), "Demo");
        // Playback
        assert_eq!(l10n.t("playback-preferences"), "Playback Preferences");
        // Playback info
        assert_eq!(l10n.t("source"), "Source");
        // Subtitle styles
        assert_eq!(l10n.t("subtitle-styles-title"), "Subtitle Styles");
        // Jellyfin
        assert_eq!(l10n.t("sign-in"), "Sign In");
        // Accessibility
        assert_eq!(l10n.t("a11y-close"), "Close");
    }
}
