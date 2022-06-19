//! Contains all manifest enums.

use std::convert::TryInto;
use std::error::Error;
use std::str::FromStr;

use parse_display::{Display, FromStr};
use serde::{Deserialize, Serialize};
use url::Url as AbsoluteUrl;

use crate::errors::ManifestError;

/// The resource URL.
///
/// It can store either the parsed absolute URL or the relative URL
/// as a string. All relative URLs and document URL in the manifest
/// can be converted to absolute URLs and parsed by calling
/// [`process`][crate::WebAppManifest::process].
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Hash)]
#[serde(untagged)]
pub enum Url {
    /// The parsed absolute URL.
    Absolute(AbsoluteUrl),

    /// The relative URL as an unparsed string.
    Relative(String),

    /// The unknown URL.
    ///
    /// What this URL represents depends on the context. When using it as
    /// a start URL, it represents the document URL. When using it as a
    /// scope URL, it represents the result of parsing `.` with the start
    /// URL as a base. In most other cases, it represents invalid URL that
    /// should not be provided, and attempting to parse/use it should
    /// cause an error.
    Unknown,
}

impl Default for Url {
    #[inline]
    fn default() -> Self {
        Self::Unknown
    }
}

impl FromStr for Url {
    type Err = Box<dyn Error>;

    #[inline]
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Ok(match AbsoluteUrl::parse(string) {
            Ok(url) => Self::Absolute(url),
            Err(_) => Self::Relative(string.to_string()),
        })
    }
}

impl TryInto<String> for Url {
    type Error = ManifestError;

    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            Self::Absolute(url) => Ok(url.into()),
            Self::Relative(url) => Ok(url),
            _ => Err(Self::Error::NotStringifyable { url: self }),
        }
    }
}

impl TryInto<AbsoluteUrl> for Url {
    type Error = ManifestError;

    fn try_into(self) -> Result<AbsoluteUrl, Self::Error> {
        match self {
            Self::Absolute(url) => Ok(url),
            _ => Err(Self::Error::NotAbsolute { url: self }),
        }
    }
}

/// The base direction in which to display direction-capable members of the manifest.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Direction {
    /// No explicit directionality.
    ///
    /// This is the default variant.
    ///
    /// The implementation should determine the text's direction of direction-capable
    /// members by applying the [Rule P1](https://www.unicode.org/reports/tr9/tr9-42.html#P1)
    /// of [Unicode Bidirectional Algorithm](https://www.unicode.org/reports/tr9/tr9-42.html).
    #[serde(rename = "auto")]
    Auto,

    /// Left-to-right text.
    ///
    /// The implementation should override the [Rule P3](https://www.unicode.org/reports/tr9/tr9-42.html#P3)
    /// of [Unicode Bidirectional Algorithm](https://www.unicode.org/reports/tr9/tr9-42.html),
    /// setting the paragraph embedding level to `0`.
    #[serde(rename = "ltr")]
    Ltr,

    /// Right-to-left text.
    ///
    /// The implementation should override the [Rule P3](https://www.unicode.org/reports/tr9/tr9-42.html#P3)
    /// of [Unicode Bidirectional Algorithm](https://www.unicode.org/reports/tr9/tr9-42.html),
    /// setting the paragraph embedding level to `1`.
    #[serde(rename = "rtl")]
    Rtl,
}

impl Default for Direction {
    #[inline]
    fn default() -> Self {
        Self::Auto
    }
}

/// The preferred display mode of the web application.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Display {
    /// Opens the web application in a conventional browser tab or new window,
    /// depending on the browser and platform.
    ///
    /// This is the default variant and must be supported by the user agent.
    #[serde(rename = "browser")]
    Browser,

    /// Opens the web application with browser UI elements hidden and takes
    /// up the entirety of the available display area.
    ///
    /// If not supported, the user agent must fall back to the `standalone` mode.
    #[serde(rename = "fullscreen")]
    Fullscreen,

    /// Opens the web application to look and feel like a standalone native
    /// application. This can include the application having a different window,
    /// its own icon in the application launcher, etc. In this mode, the user agent
    /// will exclude standard browser UI elements such as an URL bar, but can
    /// include other system UI elements such as a status bar and/or system
    /// back button.
    ///
    /// If not supported, the user agent must fall back to the `minimal-ui` mode.
    #[serde(rename = "standalone")]
    Standalone,

    /// Opens the web application to look and feel like a standalone native
    /// application, but provides the end-user with some means to access a minimal
    /// set of UI elements for controlling navigation. A user agent can include
    /// other platform specific UI elements, such as "share" and "print" buttons or
    /// whatever is customary on the platform and user agent.
    ///
    /// If not supported, the user agent must fall back to the `browser` mode.
    #[serde(rename = "minimal-ui")]
    MinimalUi,
}

impl Default for Display {
    #[inline]
    fn default() -> Self {
        Self::Browser
    }
}

/// The preferred orientation of the web application.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Orientation {
    /// Any is an orientation that means the screen can be locked to any one
    /// of the `portrait-primary`, `portrait-secondary`, `landscape-primary`
    /// and `landscape-secondary`.
    ///
    /// This is the default variant.
    #[serde(rename = "any")]
    Any,

    /// Natural is an orientation that refers to either `portrait-primary` or
    /// `landscape-primary` depending on the device's usual orientation. This
    /// orientation is usually provided by the underlying operating system.
    #[serde(rename = "natural")]
    Natural,

    /// Landscape is an orientation where the screen width is greater than the
    /// screen height.
    #[serde(rename = "landscape")]
    Landscape,

    /// Portrait is an orientation where the screen width is less than or equal
    /// to the screen height.
    #[serde(rename = "portrait")]
    Portrait,

    /// Landscape-primary is an orientation where the screen width is greater than
    /// the screen height. If the device's natural orientation is landscape, then it
    /// is in landscape-primary when held in that position.
    #[serde(rename = "landscape-primary")]
    LandscapePrimary,

    /// Landscape-secondary is an orientation where the screen width is greater than
    /// the screen height. If the device's natural orientation is landscape, then it
    /// is in landscape-secondary when rotated 180° from its natural orientation.
    #[serde(rename = "landscape-secondary")]
    LandscapeSecondary,

    /// Portrait-primary is an orientation where the screen width is less than or equal
    /// to the screen height. If the device's natural orientation is portrait, then it
    /// is in portrait-primary when held in that position.
    #[serde(rename = "portrait-primary")]
    PortraitPrimary,

    /// Portrait-secondary is an orientation where the screen width is less than or equal
    /// to the screen height. If the device's natural orientation is portrait, then it
    /// is in portrait-secondary when rotated 180° from its natural orientation.
    #[serde(rename = "portrait-secondary")]
    PortraitSecondary,
}

impl Default for Orientation {
    #[inline]
    fn default() -> Self {
        Self::Any
    }
}

/// The HTTP request method for the web share target.
#[derive(Display, FromStr, Debug, Eq, PartialEq, Clone, Copy, Hash)]
#[display(style = "UPPERCASE")]
pub enum ShareTargetMethod {
    /// The web share target uses the GET method.
    ///
    /// This is the default variant.
    #[from_str(regex = "(?i)GET")]
    Get,

    /// The web share target uses the POST method.
    #[from_str(regex = "(?i)POST")]
    Post,
}

impl Default for ShareTargetMethod {
    #[inline]
    fn default() -> Self {
        Self::Get
    }
}

/// The encoding in the body of a POST request for the web share target.
/// It is ignored when the method is GET.
#[derive(Display, FromStr, Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum ShareTargetEnctype {
    /// The web share target uses `application/x-www-form-urlencoded` encoding.
    ///
    /// This is the default variant.
    #[display("application/x-www-form-urlencoded")]
    #[from_str(regex = "(?i)application/x-www-form-urlencoded")]
    UrlEncoded,

    /// The web share target uses `multipart/form-data` encoding.
    #[display("multipart/form-data")]
    #[from_str(regex = "(?i)multipart/form-data")]
    FormData,
}

impl Default for ShareTargetEnctype {
    #[inline]
    fn default() -> Self {
        Self::UrlEncoded
    }
}

/// The size of the image.
#[derive(Display, FromStr, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
pub enum ImageSize {
    /// Image is `{0}` by `{1}` pixels big.
    #[display("{0}x{1}")]
    #[from_str(regex = "(?P<0>[0-9]+)[xX](?P<1>[0-9]+)")]
    Fixed(u32, u32),

    /// Image can support any size.
    ///
    /// This is the default variant.
    #[display("any")]
    #[from_str(regex = "(?i)any")]
    Any,
}

impl Default for ImageSize {
    #[inline]
    fn default() -> Self {
        Self::Any
    }
}

/// The purpose of the image.
#[derive(Display, FromStr, Debug, Eq, PartialEq, Clone, Copy, Hash)]
#[display(style = "snake_case")]
pub enum ImagePurpose {
    /// The user agent is free to display the icon in any context.
    ///
    /// This is the default variant.
    Any,

    /// A user agent can present this icon where a monochrome icon with a solid fill is needed.
    /// The color information in the icon is discarded and only the alpha data is used.
    Monochrome,

    /// The image is designed with icon masks and safe zone in mind, such that any part of
    /// the image that is outside the safe zone can safely be ignored and masked away by
    /// the user agent.
    Maskable,
}

impl Default for ImagePurpose {
    #[inline]
    fn default() -> Self {
        Self::Any
    }
}

#[cfg(test)]
#[allow(clippy::needless_update)]
#[rustfmt::skip::macros(assert_eq, assert_matches, assert)]
mod tests {
    use assert_matches::assert_matches;

    use super::*;

    #[test]
    fn test_absolute_url_from_string() {
        let url = "https://example.com/index.html";

        let actual = Url::from_str(url).unwrap();
        let expected = Url::Absolute(AbsoluteUrl::parse(url).unwrap());

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_relative_url_from_string() {
        let url = "/index.html";

        let actual = Url::from_str(url).unwrap();
        let expected = Url::Relative(url.to_string());

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_try_absolute_url_into_string() {
        let url = "https://example.com/handler/?protocol=%s";
        let parsed = Url::from_str(url).unwrap();

        let actual: String = parsed.try_into().unwrap();
        let expected: String = url.into();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_try_relative_url_into_string() {
        let url = "/handler/?protocol=%s";
        let parsed = Url::from_str(url).unwrap();

        let actual: String = parsed.try_into().unwrap();
        let expected: String = url.into();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_try_unknown_url_into_string() {
        self::assert_matches!(
            TryInto::<String>::try_into(Url::Unknown).unwrap_err(),
            ManifestError::NotStringifyable { url: _ }
        );
    }

    #[test]
    fn test_try_correct_url_into_absolute() {
        let url = AbsoluteUrl::parse("https://example.com").unwrap();

        let actual: AbsoluteUrl = Url::Absolute(url.clone()).try_into().unwrap();
        let expected: AbsoluteUrl = url;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_try_invalid_url_into_absolute() {
        self::assert_matches!(
            TryInto::<AbsoluteUrl>::try_into(Url::Relative("/index.html".to_string())).unwrap_err(),
            ManifestError::NotAbsolute { url: _ }
        );
    }

    #[test]
    fn test_any_image_size() {
        let deserialized = ImageSize::from_str("aNy").unwrap();
        assert_eq!(deserialized, ImageSize::Any);

        let serialized = deserialized.to_string();
        assert_eq!(serialized, "any");
    }

    #[test]
    fn test_fixed_image_size() {
        let deserialized = ImageSize::from_str("64X128").unwrap();
        assert_eq!(deserialized, ImageSize::Fixed(64, 128));

        let serialized = deserialized.to_string();
        assert_eq!(serialized, "64x128");
    }
}
