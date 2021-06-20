//! Contains all manifest resources.

use std::collections::HashSet;

use mime::MediaType;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none, DisplayFromStr};
use smart_default::SmartDefault;

use crate::types::*;

/// A fingerprint represents a set of cryptographic fingerprints used for verifying the application.
///
/// # See also
///
/// - [Specification](https://w3c.github.io/manifest/#dfn-fingerprints-0)
///
#[skip_serializing_none]
#[derive(SmartDefault, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(default)]
pub struct ExternalApplicationFingerprint {
    /// Platform-defined fingerprint type.
    pub r#type: String,

    /// Platform-defined fingerprint value.
    pub value: String,
}

/// An external application resource represents an application related to the web application.
///
/// # See also
///
/// - [Specification](https://w3c.github.io/manifest/#dfn-external-application-resource)
///
#[skip_serializing_none]
#[derive(SmartDefault, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(default)]
pub struct ExternalApplicationResource {
    /// The `platform` field represents the platform this external application resource is
    /// associated with. A platform represents a software distribution ecosystem or an
    /// operating system. The specification does not define the particular values for
    /// the platform member.
    pub platform: String,

    /// The `min_version` field represents the minimum version of the application that is
    /// considered related to this web app. This version is a string with platform-specific
    /// syntax and semantics.
    pub min_version: Option<String>,

    /// The `url` field is the URL where the application can be found. Either this field or
    /// the [`id`][ExternalApplicationResource::id] field (or both) must be set.
    pub url: Option<Url>,

    /// The `id` field represents the id which is used to represent the application on
    /// the platform. Either this field or the [`url`][ExternalApplicationResource::url]
    /// field (or both) must be set.
    pub id: Option<String>,

    /// The `fingerprints` field represents an array of fingerprints.
    pub fingerprints: Vec<ExternalApplicationFingerprint>,
}

/// A protocol resource represents a protocol that application can handle and should be registered.
#[skip_serializing_none]
#[derive(SmartDefault, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(default)]
pub struct ProtocolHandlerResource {
    /// The `protocol` field contains the protocol to be handled.
    pub protocol: String,

    /// The `url` field contains the URL within the application scope that will handle
    /// the protocol. The `%s` token should be replaced by the URL starting with the
    /// protocol handler's scheme.
    pub url: Url,
}

/// A shortcut resource represents a link to a key task or page within a web app.
///
/// # See also
///
/// - [Specification](https://w3c.github.io/manifest/#shortcut-items)
///
#[skip_serializing_none]
#[derive(SmartDefault, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(default)]
pub struct ShortcutResource {
    /// The `name` field represents the name of the shortcut as it is usually
    /// displayed to the user in a context menu.
    pub name: String,

    /// The `name` field represents the short version name of the shortcut. It is
    /// intended to be used where there is insufficient space to display the full
    /// name of the shortcut.
    pub short_name: Option<String>,

    /// The `description` field allows the developer to describe the purpose of the shortcut
    /// and may be exposed to assistive technology.
    pub description: Option<String>,

    /// The `url` field stores the URL within the application scope that opens when
    /// the shortcut is activated.
    pub url: Url,

    /// The `icons` field serves as iconic representations of the shortcut in various contexts.
    pub icons: Vec<IconResource>,
}

/// The share target params represent which parameters names should the application receive.
///
/// # See also
///
/// - [Specification](https://w3c.github.io/web-share-target/#sharetargetparams-and-its-members)
///
#[skip_serializing_none]
#[derive(SmartDefault, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(default)]
pub struct ShareTargetParams {
    /// The `title` field specifies the name of the query parameter used for
    /// the title of the document being shared.
    pub title: Option<String>,

    /// The `text` field specifies the name of the query parameter used for
    /// the arbitrary text that forms the body of the message being shared.
    pub text: Option<String>,

    /// The `url` field specifies the name of the query parameter used for
    /// the URL string referring to a resource being shared.
    pub url: Option<String>,
}

/// The share target represents how the application receives share data.
///
/// # See also
///
/// - [Specification](https://w3c.github.io/web-share-target/#sharetarget-and-its-members)
///
#[skip_serializing_none]
#[serde_as]
#[derive(SmartDefault, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(default)]
pub struct ShareTargetResource {
    /// The `action` field specifies the URL for the web share target.
    pub action: Url,

    /// The `method` field specifies the HTTP request method for the web share target.
    #[serde_as(as = "DisplayFromStr")]
    pub method: ShareTargetMethod,

    /// The `enctype` field specifies how the share data is encoded in
    /// the body of a POST request. It is ignored when method is GET.
    #[serde_as(as = "DisplayFromStr")]
    pub enctype: ShareTargetEnctype,

    /// The `params` field specifies which parameters names should the application receive.
    pub params: ShareTargetParams,
}

/// An icon resource represents an image resource that is conceptually part of a
/// web application, suitable to use in various contexts, such as application menu.
///
/// # See also
///
/// - [Specification](https://w3c.github.io/manifest/#manifest-image-resources)
///
#[skip_serializing_none]
#[derive(SmartDefault, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(default)]
pub struct IconResource {
    /// The `src` field stores the path to the image file.
    pub src: Url,

    /// The `type` field serves as a hint as to the media type of the image. Its
    /// purpose is to allow a user agent to quickly ignore images with media types
    /// it does not support.
    pub r#type: Option<MediaType>,

    /// The `sizes` field contains image dimensions. It allows a user agent to
    /// quickly ignore images with incorrect sizes for the purpose.
    #[default([ImageSize::default()].iter().cloned().collect())]
    #[serde(with = "serde_with::rust::StringWithSeparator::<serde_with::SpaceSeparator>")]
    pub sizes: HashSet<ImageSize>,

    /// The `purpose` field defines the purposes of the image.
    #[default([ImagePurpose::default()].iter().cloned().collect())]
    #[serde(with = "serde_with::rust::StringWithSeparator::<serde_with::SpaceSeparator>")]
    pub purpose: HashSet<ImagePurpose>,

    /// The `label` field represents the accessible name of the image.
    pub label: Option<String>,
}

/// A screenshots resource represents an image resource, representing the web
/// application in common usage scenarios.
///
/// # See also
///
/// - [Specification](https://w3c.github.io/manifest-app-info/#screenshot-object-and-its-members)
///
#[skip_serializing_none]
#[derive(SmartDefault, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(default)]
pub struct ScreenshotResource {
    /// The `src` field stores the path to the image file.
    pub src: Url,

    /// The `type` field serves as a hint as to the media type of the image. Its
    /// purpose is to allow a user agent to quickly ignore images with media types
    /// it does not support.
    pub r#type: Option<MediaType>,

    /// The `sizes` field contains image dimensions. It allows a user agent to
    /// quickly ignore images with incorrect sizes for the purpose.
    #[default([ImageSize::default()].iter().cloned().collect())]
    #[serde(with = "serde_with::rust::StringWithSeparator::<serde_with::SpaceSeparator>")]
    pub sizes: HashSet<ImageSize>,

    /// The `platform` field represents the distribution platform for which a
    /// given screenshot applies. User agents may show as many screenshots as
    /// they choose, but should not display screenshots that do not pertain
    /// to their platform.
    pub platform: Option<String>,

    /// The `label` field represents the accessible name of the image.
    pub label: Option<String>,
}

#[cfg(test)]
#[allow(clippy::needless_update)]
#[rustfmt::skip::macros(assert_eq, assert_matches, assert)]
mod tests {
    use super::*;

    #[test]
    fn test_share_target_method() {
        let serialized = r#"{"action":"share","method":"PoSt","params":{}}"#;
        let deserialized: ShareTargetResource = serde_json::from_str(serialized).unwrap();

        assert_eq!(deserialized.method, ShareTargetMethod::Post);
    }

    #[test]
    fn test_share_target_enctype() {
        let serialized = r#"{"action":"share","enctype":"MultiPart/Form-Data","params":{}}"#;
        let deserialized: ShareTargetResource = serde_json::from_str(serialized).unwrap();

        assert_eq!(deserialized.enctype, ShareTargetEnctype::FormData);
    }

    #[test]
    fn test_icon_sizes() {
        let icon = IconResource {
            src: Url::Relative("icon.png".to_string()),
            sizes: [ImageSize::Fixed(16, 16), ImageSize::Fixed(32, 32)].iter().cloned().collect(),
            ..Default::default()
        };

        let serialized = serde_json::to_string(&icon).unwrap();

        // Two checks because `HashSet` does not have fixed order
        let expected1 = r#"{"src":"icon.png","sizes":"16x16 32x32","purpose":"any"}"#;
        let expected2 = r#"{"src":"icon.png","sizes":"32x32 16x16","purpose":"any"}"#;
        assert!(serialized == expected1 || serialized == expected2);

        let deserialized: IconResource = serde_json::from_str(&serialized).unwrap();

        // Just check length and each size separately
        assert_eq!(deserialized.sizes.len(), 2);
        assert!(deserialized.sizes.contains(&ImageSize::Fixed(16, 16)));
        assert!(deserialized.sizes.contains(&ImageSize::Fixed(32, 32)));
    }

    #[test]
    fn test_icon_purpose() {
        let icon = IconResource {
            src: Url::Relative("icon.png".to_string()),
            purpose: [ImagePurpose::Maskable, ImagePurpose::Monochrome].iter().cloned().collect(),
            ..Default::default()
        };

        let serialized = serde_json::to_string(&icon).unwrap();

        // Two checks because `HashSet` does not have fixed order
        let expected1 = r#"{"src":"icon.png","sizes":"any","purpose":"maskable monochrome"}"#;
        let expected2 = r#"{"src":"icon.png","sizes":"any","purpose":"monochrome maskable"}"#;
        assert!(serialized == expected1 || serialized == expected2);

        let deserialized: IconResource = serde_json::from_str(&serialized).unwrap();

        // Just check length and each purpose separately
        assert_eq!(deserialized.purpose.len(), 2);
        assert!(deserialized.purpose.contains(&ImagePurpose::Maskable));
        assert!(deserialized.purpose.contains(&ImagePurpose::Monochrome));
    }

    #[test]
    fn test_screenshot_sizes() {
        let icon = ScreenshotResource {
            src: Url::Relative("screenshot.png".to_string()),
            sizes: [ImageSize::Fixed(256, 512), ImageSize::Fixed(1024, 2048)]
                .iter()
                .cloned()
                .collect(),
            ..Default::default()
        };

        let serialized = serde_json::to_string(&icon).unwrap();

        // Two checks because `HashSet` does not have fixed order
        let expected1 = r#"{"src":"screenshot.png","sizes":"256x512 1024x2048"}"#;
        let expected2 = r#"{"src":"screenshot.png","sizes":"1024x2048 256x512"}"#;
        assert!(serialized == expected1 || serialized == expected2);

        let deserialized: ScreenshotResource = serde_json::from_str(&serialized).unwrap();

        // Just check length and each size separately
        assert_eq!(deserialized.sizes.len(), 2);
        assert!(deserialized.sizes.contains(&ImageSize::Fixed(256, 512)));
        assert!(deserialized.sizes.contains(&ImageSize::Fixed(1024, 2048)));
    }
}
