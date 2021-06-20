#![warn(missing_docs)]

//! Rust data structure and utilities for (de)serialization and storage of Web Application Manifests.
//!
//! # Description
//!
//! This crate contains a [Serde-based][link-serde] data structure for parsing,
//! writing and storing [Web Application Manifests][link-mdn-manifest]. It complies
//! with the W3C specification and schema as much as possible and provides easy
//! access and type conversion for all official manifest fields. Because it uses
//! Serde, it supports much more than just JSON, so it can also be stored in more
//! efficient formats after being retrieved from the website.
//!
//! # Warning
//!
//! Due to reliance on some crates currently not published on Crates.io, this crate
//! can also not be published on Crates.io. You will need to include it through the
//! Git repository. When required crates are published on Crates.io, this crate will
//! also be published.
//!
//! # Installation
//!
//! You will need to install the crate from a Git repository. You will also need
//! to install `serde_json` crate for parsing JSON manifests, and optionally other
//! Serde-based crates for other storage formats.
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! web_app_manifest = { git = "https://github.com/filips123/WebAppManifestRS" }
//! serde_json = "1.0"
//! ```
//!
//! **Note:** It is highly recommended to also specify `rev` and/or use
//! `Cargo.lock` to prevent breaking your project on backwards-incompatible
//! changes.
//!
//! # Usage
//!
//! ## Parsing
//!
//! To parse a manifest from JSON, use any `serde_json::from_` function,
//! depending on your data source. For example, to parse a manifest from
//! a string, use `serde_json::from_str`:
//!
//! ```rust
//! use web_app_manifest::WebAppManifest;
//!
//! let json = r#"{
//!     "start_url": "/",
//!     "scope": "/",
//!     "name": "Example App",
//!     "short_name": "Example",
//!     "display": "standalone",
//!     "orientation": "portrait"
//! }"#;
//!
//! let manifest: WebAppManifest = serde_json::from_str(json)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! Because URLs can be relative and `serde_json` does not know how to resolve
//! them, you will also need to call the [`process`](WebAppManifest::process)
//! method and pas the document and manifest URLs to resolve all relative URLs
//! in the manifest and perform origin and scope validation:
//!
//! ```rust
//! # use web_app_manifest::WebAppManifest;
//! # let json = r#"{"start_url":"/","scope":"/","name":"Example App","short_name":"Example","display":"standalone","orientation":"portrait"}"#;
//! # let mut manifest: WebAppManifest = serde_json::from_str(json)?;
//! use url::Url;
//!
//! let document_url = Url::parse("https://example.com/index.html")?;
//! let manifest_url = Url::parse("https://example.com/site.webmanifest")?;
//!
//! manifest.process(&document_url, &manifest_url)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! You will now have the access to all manifest fields in the correct types:
//!
//! ```rust
//! # use web_app_manifest::WebAppManifest;
//! # let json = r#"{"start_url":"/","scope":"/","name":"Example App","short_name":"Example","display":"standalone","orientation":"portrait"}"#;
//! # let mut manifest: WebAppManifest = serde_json::from_str(json)?;
//! use web_app_manifest::types::{Display, Orientation};
//!
//! assert_eq!(manifest.name, Some("Example App".to_string()));
//! assert_eq!(manifest.short_name, Some("Example".to_string()));
//! assert_eq!(manifest.display, Display::Standalone);
//! assert_eq!(manifest.orientation, Orientation::Portrait);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Creating
//!
//! To create a new manifest, simply construct the struct:
//!
//! ```rust
//! use std::str::FromStr;
//!
//! use web_app_manifest::WebAppManifest;
//! use web_app_manifest::types::Url;
//! use web_app_manifest::resources::IconResource;
//!
//! let manifest = WebAppManifest {
//!     name: Some("Example App".to_string()),
//!     short_name: Some("Example".to_string()),
//!
//!     start_url: Url::from_str("https://example.com/app/index.html")?,
//!     scope: Url::from_str("https://example.com/app")?,
//!
//!     background_color: Some(csscolorparser::parse("rgb(100%,0%,0%)")?),
//!     theme_color: Some(csscolorparser::parse("aliceblue")?),
//!
//!     icons: vec![IconResource {
//!         src: Url::from_str("/resources/icon.png")?,
//!         ..Default::default()
//!     }],
//!
//!     ..Default::default()
//! };
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! **Important:** Always use `..Default::default()` when constructing
//! the manifest or its other structs. Adding new public fields will not
//! count as a major change, so your code could break without it.
//!
//! Processing the manifest is not necessary, because it will be processed
//! when parsing by this crate or the browser in any case.
//!
//! Then use any Serde-based function, such as `serde_json::to_string`,
//! to serialize it:
//!
//! ```rust
//! # use web_app_manifest::WebAppManifest;
//! # use web_app_manifest::types::Url;
//! # use web_app_manifest::resources::IconResource;
//! # let manifest=WebAppManifest{..Default::default()};
//! let json = serde_json::to_string(&manifest)?;
//! println!("{}", json);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Other
//!
//! See [docs][link-docs] of structs and fields for more documentation.
//! You can also check the [integration tests][link-tests] for usage examples.
//!
//! # Versioning
//!
//! This crate is currently in the alpha stage, expect backwards-incompatible
//! changes. Until it is published to Crates.io, there will be no Git tags or
//! official releases. Once it is ready to be published to Crates.io, it will
//! use a proper versioning scheme (SemVer).
//!
//! **Note:** Adding new public struct fields or enum variants, if required by
//! the W3C specification updates, will not count as a major change. Your code
//! should always be using `..Default::default()` when constructing the manifest
//! or its other structs.
//!
//! # Contributing
//!
//! Any contribution intentionally submitted for inclusion in this repository
//! shall be dual-licensed under the MIT License and Apache License 2.0,
//! without any additional terms or conditions.
//!
//! When changing the README file, edit the documentation in [`src/lib.rs`][link-lib-file]
//! instead, and use [`cargo-readme`][link-cargo-readme] to generate a README file.
//!
//! Please also make sure that your code is properly linted and formatted using
//! [clippy][link-clippy] and [rustfmt][link-rustfmt].
//!
//! # License
//!
//! This library is licensed under the MIT License or Apache License 2.0, at
//! your opinion. See the [LICENSE][link-license-file] file for more details.
//!
//! The documentation for manifest fields and enum variants is heavily based on
//! the documentation by MDN Web Docs and the W3C and WHATWG specifications,
//! licensed under the following licenses:
//!
//! - [Web App Manifests on MDN][link-mdn-manifest] by Mozilla Contributors is licensed under the [CC-BY-SA-2.5][link-license-cc-by-sa-2.5].
//! - [Web Application Manifest (W3C Editor's Draft)][link-w3c-manifest] by W3C and Editors is licensed under the [W3C Document License][link-license-w3c].
//! - [Web Application Manifest - Application Info (W3C Editor's Draft)][link-w3c-manifest-app-info] by W3C and Editors is licensed under the [W3C Document License][link-license-w3c].
//! - [The Screen Orientation API (W3C Working Draft)][link-w3c-orientation] by W3C and Editors is licensed under the [W3C Document License][link-license-w3c].
//! - [Web Share Target API (Unofficial Draft)][link-w3c-share-target] by W3C and Editors is licensed under the [W3C Document License][link-license-w3c].
//! - [HTML Living Standard][link-whatwg-html] by WHATWG and Editors is licensed under the [CC-BY-4.0][link-license-cc-by-4.0].
//!
//! **Note:** The documentation and library are not affiliated with MDN or W3C in any way.
//! Any content in this documentation may not be the correct or latest W3C specification.
//! Read the official W3C specification for the accurate data.
//!
//! [link-serde]: https://serde.rs/
//!
//! [link-mdn-manifest]: https://developer.mozilla.org/en-US/docs/Web/Manifest
//! [link-w3c-manifest]: https://w3c.github.io/manifest/
//! [link-w3c-manifest-app-info]: https://w3c.github.io/manifest-app-info/
//! [link-w3c-orientation]: https://www.w3.org/TR/screen-orientation/
//! [link-w3c-share-target]: https://w3c.github.io/web-share-target/
//! [link-whatwg-html]: https://html.spec.whatwg.org/multipage/semantics.html
//!
//! [link-docs]: https://docs.rs/web_app_manifest
//! [link-tests]: https://github.com/filips123/WebAppManifestRS/blob/main/tests/tests.rs
//!
//! [link-lib-file]: https://github.com/filips123/WebAppManifestRS/blob/main/src/lib.rs
//! [link-license-file]: https://github.com/filips123/WebAppManifestRS/blob/main/LICENSE
//!
//! [link-license-cc-by-sa-2.5]: https://creativecommons.org/licenses/by-sa/2.5/
//! [link-license-cc-by-4.0]: https://creativecommons.org/licenses/by/4.0/
//! [link-license-w3c]: https://www.w3.org/Consortium/Legal/2015/doc-license
//!
//! [link-cargo-readme]: https://github.com/livioribeiro/cargo-readme
//! [link-clippy]: https://github.com/rust-lang/rust-clippy
//! [link-rustfmt]: https://github.com/rust-lang/rustfmt

use csscolorparser::Color;
use language_tags::LanguageTag;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use smart_default::SmartDefault;
use url::Url as AbsoluteUrl;

use crate::errors::ManifestError;
use crate::resources::*;
use crate::types::*;

pub mod errors;
pub mod resources;
pub mod types;

/// A manifest is a JSON document that contains startup parameters and
/// application defaults for when a web application is launched.
///
/// See the [main crate documentation][crate] for more details about usage,
/// and the fields, types and resources documentations for more details
/// about specific fields and their use-cases.
#[skip_serializing_none]
#[derive(SmartDefault, Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(default)]
pub struct WebAppManifest {
    /// The `start_url` field represents the start URL of the web application, which is the
    /// preferred URL that should be loaded when the user launches the web application.
    ///
    /// The `start_url` field is purely advisory, and a user agent may ignore it or provide
    /// the end-user the choice not to make use of it. A user agent may also allow the
    /// end-user to modify the URL when, for instance, a bookmark for the web application
    /// is being created or any time thereafter.
    ///
    /// By default, it will be set to document URL. It can be converted to parsed absolute URL
    /// by calling [`process`][WebAppManifest::process].
    ///
    /// # See also
    ///
    /// - [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/Manifest/start_url)
    /// - [Specification](https://w3c.github.io/manifest/#start_url-member)
    ///
    pub start_url: Url,

    /// The `scope` field defines the navigation scope of this web application's application
    /// context. It restricts what web pages can be viewed while the manifest is applied.
    ///
    /// If the user navigates outside the scope, it reverts to a normal web page inside
    /// a browser tab or window.
    ///
    /// By default, it will be set to the result of parsing `.` with the start URL as a base.
    /// It can be converted to parsed absolute URL by calling [`process`][WebAppManifest::process].
    ///
    /// # See also
    ///
    /// - [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/Manifest/scope)
    /// - [Specification](https://w3c.github.io/manifest/#scope-member)
    ///
    pub scope: Url,

    /// The `name` field represents the name of the web application as it is usually
    /// displayed to the user.
    ///
    /// It is directionality-capable, which means it can be displayed left-to-right
    /// or right-to-left based on the values of the [`dir`][WebAppManifest::dir] and
    /// [`lang`][WebAppManifest::lang] fields.
    ///
    /// # See also
    ///
    /// - [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/Manifest/name)
    /// - [Specification](https://w3c.github.io/manifest/#name-member)
    ///
    pub name: Option<String>,

    /// The `short_name` field represents the name of the web application displayed to the
    /// user if there is not enough space to display [`name`][WebAppManifest::name].
    ///
    /// It is directionality-capable, which means it can be displayed left-to-right
    /// or right-to-left based on the values of the [`dir`][WebAppManifest::dir] and
    /// [`lang`][WebAppManifest::lang] fields.
    ///
    /// # See also
    ///
    /// - [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/Manifest/short_name)
    /// - [Specification](https://w3c.github.io/manifest/#short_name-member)
    ///
    pub short_name: Option<String>,

    /// The `description` member allows the developer to describe the purpose of the
    /// web application. It serves as the accessible description of an installed web
    /// application.
    ///
    /// It is directionality-capable, which means it can be displayed left-to-right
    /// or right-to-left based on the values of the [`dir`][WebAppManifest::dir] and
    /// [`lang`][WebAppManifest::lang] fields.
    ///
    /// # See also
    ///
    /// - [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/Manifest/description)
    /// - [Specification](https://w3c.github.io/manifest-app-info/#description-member)
    ///
    pub description: Option<String>,

    /// The `categories` field describes the application categories to which the web
    /// application belongs.
    ///
    /// It is meant as a hint to catalogs or stores listing web applications. Catalogs
    /// and stores are not required to honor this hint. Most stores and catalogs lower-case
    /// the categories before processing. Manifest authors are encouraged to use lower-case
    /// in the first place.
    ///
    /// There is no standard list of possible values, but the W3C maintains a list
    /// of known categories.
    ///
    /// # See also
    ///
    /// - [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/Manifest/categories)
    /// - [Specification](https://w3c.github.io/manifest-app-info/#categories-member)
    ///
    pub categories: Vec<String>,

    /// The `keywords` field describes the application keywords which may be used in
    /// addition to other metadata to provide more information about the application.
    ///
    /// Unlike categories, it is meant to contain specific arbitrary keywords that can
    /// describe the application and its purpose in-depth.
    ///
    /// It is meant as a hint to catalogs or stores listing web applications. Catalogs
    /// and stores are not required to honor this hint. It can also be used in the system
    /// application menu/search to facilitate searching through installed applications.
    ///
    /// *Note:* This field is currently not described in the specification and is not standard.
    ///
    pub keywords: Vec<String>,

    /// The `dir` field describes the base direction in which to display direction-capable
    /// members of the manifest. Together with the [`lang`][WebAppManifest::lang] field,
    /// it helps to correctly display right-to-left languages.
    ///
    /// # See also
    ///
    /// - [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/Manifest/dir)
    /// - [Specification](https://w3c.github.io/manifest/#dir-member)
    ///
    pub dir: Direction,

    /// The `lang` field contains a single language tag. It specifies the primary language
    /// for the values of the manifest's directionality-capable members, and together with
    /// the [`dir`][WebAppManifest::dir] determines their directionality.
    ///
    /// # See also
    ///
    /// - [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/Manifest/lang)
    /// - [Specification](https://w3c.github.io/manifest/#lang-member)
    ///
    pub lang: Option<LanguageTag>,

    /// The `display` member determines the developers’ preferred display mode for the
    /// website. The display mode changes how much of browser UI is shown to the user
    /// and can range from `browser` (when the full browser window is shown) to
    /// `fullscreen` (when the app is full-screened).
    ///
    /// # See also
    ///
    /// - [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/Manifest/display)
    /// - [Specification](https://w3c.github.io/manifest/#display-member)
    ///
    pub display: Display,

    /// The `orientation` field defines the default orientation for all the website's
    /// top-level browsing contexts. This field and/or its specific values might not be
    /// supported by a user agent on various display modes because supporting them
    /// does not make sense for the particular context.
    ///
    /// # See also
    ///
    /// - [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/Manifest/orientation)
    /// - [Specification](https://w3c.github.io/manifest/#orientation-member)
    ///
    pub orientation: Orientation,

    /// The `background_color` field defines a placeholder background color for the
    /// application page to display before its stylesheet is loaded. This value is used
    /// by the user agent to draw the background color of a shortcut when the manifest
    /// is available before the stylesheet has loaded.
    ///
    /// It is only meant to improve the user experience while a web application is
    /// loading and must not be used by the user agent as the background color when
    /// the web application's stylesheet is available.
    ///
    /// Implementors may override the value defined by the `background_color` field
    /// to support `prefers-color-scheme`.
    ///
    /// # See also
    ///
    /// - [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/Manifest/background_color)
    /// - [Specification](https://w3c.github.io/manifest/#background_color-member)
    ///
    pub background_color: Option<Color>,

    /// The `theme_color` field defines the default theme color for the application.
    /// It can serve as the theme color for all browsing contexts to which the manifest
    /// is applied.
    ///
    /// Implementors may ignore the theme color's alpha component based on the context.
    /// Implementors may also override the theme color to support `prefers-color-scheme`.
    ///
    /// # See also
    ///
    /// - [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/Manifest/theme_color)
    /// - [Specification](https://w3c.github.io/manifest/#theme_color-member)
    ///
    pub theme_color: Option<Color>,

    /// The `iarc_rating_id` field represents the [International Age Rating Coalition (IARC)](https://www.globalratings.com/)
    /// certification code of the web application. It is intended to be used to determine
    /// which ages the web application is appropriate for.
    ///
    /// # See also
    ///
    /// - [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/Manifest/iarc_rating_id)
    /// - [Specification](https://w3c.github.io/manifest-app-info/#iarc_rating_id-member)
    ///
    pub iarc_rating_id: Option<String>,

    /// The `prefer_related_applications` field is used as a hint for the user agent to say
    /// that related applications should be preferred over the web application. If it is set
    /// to `true`, and the user agent wants to suggest to install the web application, the user
    /// agent might want to suggest installing one of the related applications instead.
    ///
    /// # See also
    ///
    /// - [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/Manifest/prefer_related_applications)
    /// - [Specification](https://w3c.github.io/manifest/#prefer_related_applications-member)
    ///
    pub prefer_related_applications: bool,

    /// The `related_applications` field specifies native applications that are installable by,
    /// or accessible to, the underlying platform. Such applications are intended to be
    /// alternatives to the manifest's website that provides similar/equivalent functionality.
    ///
    /// # See also
    ///
    /// - [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/Manifest/related_applications)
    /// - [Specification](https://w3c.github.io/manifest/#related_applications-member)
    ///
    pub related_applications: Vec<ExternalApplicationResource>,

    /// The `protocol_handlers` field specifies the protocols which this web app can register
    /// and handle. Protocol handlers register the application in an OS's application preferences.
    ///
    /// After registering a web app as a protocol handler, when a user clicks on a hyperlink with
    /// a specific scheme such as `mailto://` or `web+music://` from a browser or native app,
    /// the registered PWA would open and receive the URL.
    ///
    /// # See also
    ///
    /// - [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/Manifest/protocol_handlers)
    ///
    pub protocol_handlers: Vec<ProtocolHandlerResource>,

    /// The `shortcuts` field defines shortcuts or links to key tasks or pages within a web app.
    /// A user agent can use these values to assemble a context menu to be displayed by the OS
    /// when a user engages with the web app's icon. When user invokes a shortcut, the user agent
    /// will navigate to the address given by shortcut's url member.
    ///
    /// # See also
    ///
    /// - [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/Manifest/shortcuts)
    /// - [Specification](https://w3c.github.io/manifest/#shortcuts-member)
    ///
    pub shortcuts: Vec<ShortcutResource>,

    /// The `share_target` field declares this application to be a web share target, and describes
    /// how the application receives share data. A web share target is a type of share target.
    ///
    /// # See also
    ///
    /// - [Specification](https://w3c.github.io/web-share-target/#share_target-member)
    ///
    pub share_target: Option<ShareTargetResource>,

    /// The `icons` field specifies image files that can serve as application icons for different
    /// contexts. For example, they can be used to represent the web application amongst a list
    /// of other applications, or to integrate the web application with an OS's task switcher
    /// and/or system preferences.
    ///
    /// # See also
    ///
    /// - [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/Manifest/icons)
    /// - [Specification](https://w3c.github.io/manifest/#icons-member)
    ///
    pub icons: Vec<IconResource>,

    /// The `screenshots` field defines an array of screenshots intended to showcase the
    /// application. These images are intended to be used by progressive web app stores.
    ///
    /// # See also
    ///
    /// - [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/Manifest/screenshots)
    /// - [Specification](https://w3c.github.io/manifest-app-info/#screenshots-member)
    ///
    pub screenshots: Vec<ScreenshotResource>,
}

impl WebAppManifest {
    /// Processes the web app manifests.
    ///
    /// Processing the manifest parses all relative URLs in the manifests with
    /// either the document or manifest URL as a base, depending on context
    /// as defined in the specification.
    ///
    /// It also checks if required URLs are in the same origin and within
    /// the application scope.
    ///
    /// # Parameters
    ///
    /// - `document_url`: The main document URL from which the manifest parsing was triggered.
    ///                   It is used to determine the start URL if it is unknown and check
    ///                   if the start URL is in the correct origin.
    /// - `manifest_url`: The manifest URL. It is used as a base URL for all relative URLs
    ///                   specified in the manifest.
    ///
    /// # Returns
    ///
    /// - `Ok`: A reference to the current manifest object.
    /// - `Err`: An error in case of error while processing the manifest.
    ///
    /// # Errors
    ///
    /// - [`ManifestError`][ManifestError] if the error occurs while processing the manifest.
    ///
    /// # See also
    ///
    /// - [Specification](https://w3c.github.io/manifest/#processing)
    ///
    pub fn process(
        &mut self,
        document_url: &AbsoluteUrl,
        manifest_url: &AbsoluteUrl,
    ) -> Result<&mut Self, ManifestError> {
        // Parse the start URL either as relative URL with manifest URL as a base or as document URL
        if let Url::Relative(start_url) = &self.start_url {
            self.start_url = Url::Absolute(manifest_url.join(start_url)?);
        } else if let Url::Unknown = &self.start_url {
            self.start_url = Url::Absolute(document_url.clone());
        }

        // Parse the relative scope with the manifest URL as a base or `.` with the start URL as a base
        if let Url::Relative(scope) = &self.scope {
            self.scope = Url::Absolute(manifest_url.join(scope)?);
        } else if let Url::Unknown = &self.scope {
            let start_url = if let Url::Absolute(start_url) = &self.start_url {
                start_url
            } else {
                unreachable!()
            };
            self.scope = Url::Absolute(start_url.join(".")?);
        }

        // Parse the relative URLs in external application resources with the manifest URL as a base
        for external_application in &mut self.related_applications {
            if let Some(url) = &external_application.url {
                if let Url::Relative(url) = url {
                    external_application.url = Some(Url::Absolute(manifest_url.join(url)?));
                } else if let Url::Unknown = url {
                    return Err(ManifestError::InvalidUnknownUrl);
                }
            }
        }

        // Parse the relative URLs in protocol handler resources with the manifest URL as a base
        for protocol_handler in &mut self.protocol_handlers {
            if let Url::Relative(url) = &protocol_handler.url {
                protocol_handler.url = Url::Absolute(manifest_url.join(url)?);
            } else if let Url::Unknown = protocol_handler.url {
                return Err(ManifestError::InvalidUnknownUrl);
            }
        }

        // Parse the relative URLs in shortcut resources and their icons with the manifest URL as a base
        for shortcut in &mut self.shortcuts {
            if let Url::Relative(url) = &shortcut.url {
                shortcut.url = Url::Absolute(manifest_url.join(url)?);
            } else if let Url::Unknown = shortcut.url {
                return Err(ManifestError::InvalidUnknownUrl);
            }

            for shortcut_icon in &mut shortcut.icons {
                if let Url::Relative(src) = &shortcut_icon.src {
                    shortcut_icon.src = Url::Absolute(manifest_url.join(src)?);
                } else if let Url::Unknown = shortcut_icon.src {
                    return Err(ManifestError::InvalidUnknownUrl);
                }
            }
        }

        // Parse the relative share target URL with the manifest URL as a base
        if let Some(share_target) = &mut self.share_target {
            if let Url::Relative(url) = &share_target.action {
                share_target.action = Url::Absolute(manifest_url.join(url)?);
            } else if let Url::Unknown = share_target.action {
                return Err(ManifestError::InvalidUnknownUrl);
            }
        }

        // Parse the relative URLs in icon resources with the manifest URL as a base
        for icon in &mut self.icons {
            if let Url::Relative(src) = &icon.src {
                icon.src = Url::Absolute(manifest_url.join(src)?);
            } else if let Url::Unknown = icon.src {
                return Err(ManifestError::InvalidUnknownUrl);
            }
        }

        // Parse the relative URLs in screenshot resources with the manifest URL as a base
        for screenshot in &mut self.screenshots {
            if let Url::Relative(src) = &screenshot.src {
                screenshot.src = Url::Absolute(manifest_url.join(src)?);
            } else if let Url::Unknown = screenshot.src {
                return Err(ManifestError::InvalidUnknownUrl);
            }
        }

        // Get the parsed absolute scope URL
        let scope = if let Url::Absolute(scope) = &self.scope {
            scope
        } else {
            unreachable!()
        };

        // Check if the start URL is the same origin as document URL and is within the scope
        let start_url = if let Url::Absolute(start_url) = &self.start_url {
            start_url
        } else {
            unreachable!()
        };

        if start_url.origin() != document_url.origin() {
            return Err(ManifestError::NotSameOrigin {
                url1: start_url.clone(),
                url2: document_url.clone(),
            });
        }

        if start_url.origin() != scope.origin() || !start_url.path().starts_with(scope.path()) {
            return Err(ManifestError::NotWithinScope {
                url: start_url.clone(),
                scope: scope.clone(),
            });
        }

        // Check if protocol handler URLs are within the scope
        for protocol_handler in &self.protocol_handlers {
            let protocol_handler_url =
                if let Url::Absolute(protocol_handler_url) = &protocol_handler.url {
                    protocol_handler_url
                } else {
                    unreachable!()
                };

            if protocol_handler_url.origin() != scope.origin()
                || !protocol_handler_url.path().starts_with(scope.path())
            {
                return Err(ManifestError::NotWithinScope {
                    url: protocol_handler_url.clone(),
                    scope: scope.clone(),
                });
            }
        }

        // Check if shortcut URLs are within the scope
        for shortcut in &self.shortcuts {
            let shortcut_url = if let Url::Absolute(shortcut_url) = &shortcut.url {
                shortcut_url
            } else {
                unreachable!()
            };

            if shortcut_url.origin() != scope.origin()
                || !shortcut_url.path().starts_with(scope.path())
            {
                return Err(ManifestError::NotWithinScope {
                    url: shortcut_url.clone(),
                    scope: scope.clone(),
                });
            }
        }

        // Check if the share target URL is within the scope
        if let Some(share_target) = &mut self.share_target {
            let action = if let Url::Absolute(action) = &share_target.action {
                action
            } else {
                unreachable!()
            };

            if action.origin() != scope.origin() || !action.path().starts_with(scope.path()) {
                return Err(ManifestError::NotWithinScope {
                    url: action.clone(),
                    scope: scope.clone(),
                });
            }
        }

        Ok(self)
    }
}

#[cfg(test)]
#[allow(clippy::needless_update)]
#[rustfmt::skip::macros(assert_eq, assert_matches, assert)]
mod tests {
    use assert_matches::assert_matches;
    use parameterized::parameterized;

    use super::*;

    #[test]
    fn test_simple_serialization() {
        let manifest = WebAppManifest {
            start_url: Url::Relative("/hello.html".to_string()),
            name: Some("Example App".to_string()),
            short_name: Some("Example".to_string()),
            ..Default::default()
        };

        let serialized = serde_json::to_string(&manifest).unwrap();

        assert_eq!(
            serialized,
            r#"{"start_url":"/hello.html","scope":null,"name":"Example App","short_name":"Example","categories":[],"keywords":[],"dir":"auto","display":"browser","orientation":"any","prefer_related_applications":false,"related_applications":[],"protocol_handlers":[],"shortcuts":[],"icons":[],"screenshots":[]}"#
        );
    }

    #[test]
    fn test_simple_deserialization() {
        let serialized = r#"
            {
                "start_url": "/",
                "scope": "/",
                "name": "Example App",
                "short_name": "Example"
            }
        "#;

        let manifest: WebAppManifest = serde_json::from_str(serialized).unwrap();

        // Provided fields
        assert_eq!(manifest.start_url, Url::Relative("/".to_string()));
        assert_eq!(manifest.scope, Url::Relative("/".to_string()));
        assert_eq!(manifest.name, Some("Example App".to_string()));
        assert_eq!(manifest.short_name, Some("Example".to_string()));

        // Default fields
        assert_eq!(manifest.description, None);
        assert_eq!(manifest.categories.len(), 0);
        assert_eq!(manifest.dir, Direction::Auto);
        assert_eq!(manifest.lang, None);
        assert_eq!(manifest.display, Display::Browser);
        assert_eq!(manifest.orientation, Orientation::Any);
        assert_eq!(manifest.background_color, None);
        assert_eq!(manifest.theme_color, None);
        assert_eq!(manifest.iarc_rating_id, None);
        assert_eq!(manifest.prefer_related_applications, false);
        assert_eq!(manifest.related_applications.len(), 0);
        assert_eq!(manifest.protocol_handlers.len(), 0);
        assert_eq!(manifest.shortcuts.len(), 0);
        assert_eq!(manifest.icons.len(), 0);
        assert_eq!(manifest.screenshots.len(), 0);
    }

    #[test]
    fn test_absolute_url_reserialization() {
        let original = WebAppManifest {
            start_url: Url::Absolute(AbsoluteUrl::parse("https://example.com/test.html").unwrap()),
            scope: Url::Absolute(AbsoluteUrl::parse("https://example.com").unwrap()),
            ..Default::default()
        };

        let serialized = serde_json::to_string(&original).unwrap();
        let deserialized: WebAppManifest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original.start_url, Url::Absolute(AbsoluteUrl::parse("https://example.com/test.html").unwrap()));
        assert_eq!(original.start_url, deserialized.start_url);

        assert_eq!(original.scope, Url::Absolute(AbsoluteUrl::parse("https://example.com").unwrap()));
        assert_eq!(original.scope, deserialized.scope);
    }

    #[test]
    fn test_relative_url_reserialization() {
        let original = WebAppManifest {
            start_url: Url::Relative("/start".to_string()),
            scope: Url::Relative("/".to_string()),
            ..Default::default()
        };

        let serialized = serde_json::to_string(&original).unwrap();
        let deserialized: WebAppManifest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original.start_url, Url::Relative("/start".to_string()));
        assert_eq!(original.start_url, deserialized.start_url);

        assert_eq!(original.scope, Url::Relative("/".to_string()));
        assert_eq!(original.scope, deserialized.scope);
    }

    #[test]
    fn test_unknown_url_reserialization() {
        let original = WebAppManifest {
            ..Default::default()
        };

        let serialized = serde_json::to_string(&original).unwrap();
        let deserialized: WebAppManifest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original.start_url, Url::Unknown);
        assert_eq!(original.start_url, deserialized.start_url);

        assert_eq!(original.scope, Url::Unknown);
        assert_eq!(original.scope, deserialized.scope);
    }

    #[test]
    fn test_process_manifest_absolute_urls() {
        let base = AbsoluteUrl::parse("https://example.com").unwrap();

        let document_url = base.clone();
        let manifest_url = base.join("manifest.webmanifest").unwrap();

        let mut manifest = WebAppManifest {
            start_url: Url::Absolute(base.join("hello.html").unwrap()),
            scope: Url::Absolute(base.clone()),
            ..Default::default()
        };

        manifest.process(&document_url, &manifest_url).unwrap();

        assert_eq!(manifest.start_url, Url::Absolute(base.join("hello.html").unwrap()));
        assert_eq!(manifest.scope, Url::Absolute(base));
    }

    #[test]
    fn test_process_manifest_relative_urls() {
        let base = AbsoluteUrl::parse("https://example.com").unwrap();

        let document_url = base.clone();
        let manifest_url = base.join("resources/manifest.webmanifest").unwrap();

        let mut manifest = WebAppManifest {
            start_url: Url::Relative("../example.html".to_string()),
            scope: Url::Relative("..".to_string()),

            related_applications: vec![ExternalApplicationResource {
                platform: "webapp".to_string(),
                url: Some(Url::Relative("../another.html".to_string())),
                ..Default::default()
            }],

            protocol_handlers: vec![ProtocolHandlerResource {
                protocol: "mailto".to_string(),
                url: Url::Relative("../handler.html?uri=%s".to_string()),
                ..Default::default()
            }],

            shortcuts: vec![ShortcutResource {
                name: "Example Shortcut".to_string(),
                url: Url::Relative("../shortcut.html".to_string()),
                icons: vec![IconResource {
                    src: Url::Relative("shortcut.png".to_string()),
                    ..Default::default()
                }],
                ..Default::default()
            }],

            share_target: Some(ShareTargetResource {
                action: Url::Relative("../share.html".to_string()),
                ..Default::default()
            }),

            icons: vec![IconResource {
                src: Url::Relative("icon.png".to_string()),
                ..Default::default()
            }],

            screenshots: vec![ScreenshotResource {
                src: Url::Relative("screenshot.png".to_string()),
                ..Default::default()
            }],

            ..Default::default()
        };

        manifest.process(&document_url, &manifest_url).unwrap();

        assert_eq!(manifest.start_url, Url::Absolute(manifest_url.join("../example.html").unwrap()));
        assert_eq!(manifest.scope, Url::Absolute(manifest_url.join("..").unwrap()));

        assert_eq!(manifest.related_applications[0].url, Some(Url::Absolute(manifest_url.join("../another.html").unwrap())));
        assert_eq!(manifest.protocol_handlers[0].url, Url::Absolute(manifest_url.join("../handler.html?uri=%s").unwrap()));
        assert_eq!(manifest.shortcuts[0].url, Url::Absolute(manifest_url.join("../shortcut.html").unwrap()));
        assert_eq!(manifest.shortcuts[0].icons[0].src, Url::Absolute(manifest_url.join("shortcut.png").unwrap()));
        assert_eq!(manifest.share_target.unwrap().action, Url::Absolute(manifest_url.join("../share.html").unwrap()));
        assert_eq!(manifest.icons[0].src, Url::Absolute(manifest_url.join("icon.png").unwrap()));
        assert_eq!(manifest.screenshots[0].src, Url::Absolute(manifest_url.join("screenshot.png").unwrap()));
    }

    #[test]
    fn test_process_manifest_correct_unknown_urls() {
        let base = AbsoluteUrl::parse("https://example.com").unwrap();

        let document_url = base.join("index.html").unwrap();
        let manifest_url = base.join("resources/manifest.webmanifest").unwrap();

        let mut manifest = WebAppManifest {
            ..Default::default()
        };

        manifest.process(&document_url, &manifest_url).unwrap();

        assert_eq!(manifest.start_url, Url::Absolute(document_url));
        assert_eq!(manifest.scope, Url::Absolute(base));
    }

    #[parameterized(manifest = {
        &mut WebAppManifest { related_applications: vec![ExternalApplicationResource { url: Some(Url::Unknown), ..Default::default() }], ..Default::default() },
        &mut WebAppManifest { protocol_handlers: vec![ProtocolHandlerResource { url: Url::Unknown, ..Default::default() }], ..Default::default() },
        &mut WebAppManifest { shortcuts: vec![ShortcutResource { url: Url::Unknown, ..Default::default() }], ..Default::default() },
        &mut WebAppManifest { shortcuts: vec![ShortcutResource { url: Url::Relative(".".to_string()), icons: vec![IconResource { ..Default::default() }], ..Default::default() }], ..Default::default() },
        &mut WebAppManifest { share_target: Some(ShareTargetResource { action: Url::Unknown, ..Default::default() }), ..Default::default() },
        &mut WebAppManifest { icons: vec![IconResource { src: Url::Unknown, ..Default::default() }], ..Default::default() },
        &mut WebAppManifest { screenshots: vec![ScreenshotResource { src: Url::Unknown, ..Default::default() }], ..Default::default() },
    })]
    fn test_process_manifest_invalid_unknown_urls(manifest: &mut WebAppManifest) {
        let base = AbsoluteUrl::parse("https://example.com").unwrap();

        let document_url = base.join("index.html").unwrap();
        let manifest_url = base.join("manifest.webmanifest").unwrap();

        assert_eq!(
            ManifestError::InvalidUnknownUrl,
            manifest.process(&document_url, &manifest_url).unwrap_err()
        );
    }

    #[test]
    fn test_invalid_start_url_origin() {
        let base = AbsoluteUrl::parse("https://example.com").unwrap();

        let document_url = base.join("index.html").unwrap();
        let manifest_url = base.join("manifest.webmanifest").unwrap();

        let mut manifest = WebAppManifest {
            start_url: Url::Absolute(AbsoluteUrl::parse("https://example.org").unwrap()),
            ..Default::default()
        };

        self::assert_matches!(
            manifest.process(&document_url, &manifest_url).unwrap_err(),
            ManifestError::NotSameOrigin { url1: _, url2: _ }
        );
    }

    #[test]
    fn test_invalid_start_url_scope() {
        let base = AbsoluteUrl::parse("https://example.com").unwrap();

        let document_url = base.join("index.html").unwrap();
        let manifest_url = base.join("manifest.webmanifest").unwrap();

        let mut manifest = WebAppManifest {
            start_url: Url::Absolute(AbsoluteUrl::parse("https://example.com").unwrap()),
            scope: Url::Absolute(AbsoluteUrl::parse("https://example.com/scope").unwrap()),
            ..Default::default()
        };

        self::assert_matches!(
            manifest.process(&document_url, &manifest_url).unwrap_err(),
            ManifestError::NotWithinScope { url: _, scope: _ }
        );
    }

    #[parameterized(manifest = {
        &mut WebAppManifest {
            protocol_handlers: vec![ProtocolHandlerResource {
                url: Url::Absolute(AbsoluteUrl::parse("https://example.org").unwrap()),
                ..Default::default()
            }],
            ..Default::default()
        },
        &mut WebAppManifest {
            shortcuts: vec![ShortcutResource {
                url: Url::Absolute(AbsoluteUrl::parse("https://example.org").unwrap()),
                ..Default::default()
            }],
            ..Default::default()
        },
        &mut WebAppManifest {
            share_target: Some(ShareTargetResource {
                action: Url::Absolute(AbsoluteUrl::parse("https://example.org").unwrap()),
                ..Default::default()
            }),
            ..Default::default()
        },
    })]
    fn test_invalid_resource_scopes(manifest: &mut WebAppManifest) {
        let base = AbsoluteUrl::parse("https://example.com").unwrap();

        let document_url = base.join("index.html").unwrap();
        let manifest_url = base.join("manifest.webmanifest").unwrap();

        self::assert_matches!(
            manifest.process(&document_url, &manifest_url).unwrap_err(),
            ManifestError::NotWithinScope { url: _, scope: _ }
        );
    }
}
