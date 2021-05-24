use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::str::FromStr;

use csscolorparser::Color;
use language_tags::LanguageTag;
use mime::MediaType;
use url::Url as AbsoluteUrl;

use web_app_manifest::resources::{
    ExternalApplicationFingerprint, ExternalApplicationResource, IconResource,
    ProtocolHandlerResource, ScreenshotResource, ShortcutResource,
};
use web_app_manifest::types::{Display, ImagePurpose, ImageSize, Orientation, Url};
use web_app_manifest::WebAppManifest;

#[test]
#[allow(clippy::needless_update)]
#[rustfmt::skip::macros(assert_eq, assert_matches, assert)]
fn test_creating_manifest() {
    let manifest = WebAppManifest {
        start_url: Url::from_str("https://example.com/app.html").unwrap(),
        scope: Url::from_str("https://example.com/").unwrap(),
        name: Some("Example Application".to_string()),
        short_name: Some("Example".to_string()),
        lang: Some(LanguageTag::parse("en-us").unwrap()),
        display: Display::Fullscreen,
        orientation: Orientation::Landscape,

        related_applications: vec![ExternalApplicationResource {
            platform: "windows".to_string(),
            id: Some("00000000-0000-0000-0000-000000000000".to_string()),
            fingerprints: vec![ExternalApplicationFingerprint {
                r#type: "type".to_string(),
                value: "value".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        }],

        protocol_handlers: vec![ProtocolHandlerResource {
            protocol: "mailto".to_string(),
            url: Url::from_str("/mailto?url=%s").unwrap(),
            ..Default::default()
        }],

        shortcuts: vec![ShortcutResource {
            name: "Example Shortcut".to_string(),
            url: Url::from_str("/shortcut").unwrap(),
            ..Default::default()
        }],

        icons: vec![IconResource {
            src: Url::from_str("/favicon.ico").unwrap(),
            r#type: Some(MediaType::from_str("image/x-icon").unwrap()),
            sizes: [ImageSize::Fixed(64, 64)].iter().cloned().collect(),
            purpose: [ImagePurpose::Maskable].iter().cloned().collect(),
            ..Default::default()
        }],

        screenshots: vec![ScreenshotResource {
            src: Url::from_str("/screenshot.png").unwrap(),
            platform: Some("windows".to_string()),
            label: Some("This is screenshot".to_string()),
            ..Default::default()
        }],

        ..Default::default()
    };

    let mut filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    filename.push("tests/creating.webmanifest");

    let actual = serde_json::to_string_pretty(&manifest).unwrap() + "\n";
    let expected = fs::read_to_string(filename).unwrap();

    assert_eq!(actual, expected);
}

#[test]
#[allow(clippy::needless_update)]
#[rustfmt::skip::macros(assert_eq, assert_matches, assert)]
fn test_parsing_manifest() {
    let mut filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    filename.push("tests/parsing.webmanifest");

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut manifest: WebAppManifest = serde_json::from_reader(reader).unwrap();

    manifest
        .process(
            &AbsoluteUrl::parse("https://example.com/index.html").unwrap(),
            &AbsoluteUrl::parse("https://example.com/app.webmanifest").unwrap(),
        )
        .unwrap();

    assert_eq!(manifest.start_url, Url::from_str("https://example.com/app.html").unwrap());
    assert_eq!(manifest.scope, Url::from_str("https://example.com").unwrap());
    assert_eq!(manifest.lang, Some(LanguageTag::parse("en").unwrap()));
    assert_eq!(manifest.name, Some("Example Application".to_string()));
    assert_eq!(manifest.short_name, Some("Example".to_string()));
    assert_eq!(manifest.description, Some("This is some simple example web application".to_string()));
    assert_eq!(manifest.categories, vec!["example", "test"]);
    assert_eq!(manifest.display, Display::Standalone);
    assert_eq!(manifest.orientation, Orientation::Portrait);
    assert_eq!(manifest.background_color, Some(Color::from_str("aqua").unwrap()));
    assert_eq!(manifest.theme_color, Some(Color::from_str("aliceblue").unwrap()));

    assert_eq!(manifest.related_applications.len(), 2);
    assert_eq!(manifest.related_applications[0].platform, "windows");
    assert_eq!(manifest.related_applications[0].id, Some("00000000-0000-0000-0000-000000000000".to_string()));
    assert_eq!(manifest.related_applications[0].fingerprints.len(), 1);
    assert_eq!(manifest.related_applications[0].fingerprints[0].r#type, "secure");
    assert_eq!(manifest.related_applications[0].fingerprints[0].value, "something");
    assert_eq!(manifest.related_applications[1].platform, "play");
    assert_eq!(manifest.related_applications[1].min_version, Some("1.0.0".to_string()));
    assert_eq!(manifest.related_applications[1].url, Some(Url::from_str("https://play.google.com").unwrap()));
    assert_eq!(manifest.related_applications[1].fingerprints.len(), 0);

    assert_eq!(manifest.protocol_handlers.len(), 1);
    assert_eq!(manifest.protocol_handlers[0].protocol, "mailto");
    assert_eq!(manifest.protocol_handlers[0].url, Url::from_str("https://example.com/mailto?url=%s").unwrap());

    assert_eq!(manifest.shortcuts.len(), 1);
    assert_eq!(manifest.shortcuts[0].name, "Example Shortcut");
    assert_eq!(manifest.shortcuts[0].url, Url::from_str("https://example.com/shortcut").unwrap());
    assert_eq!(manifest.shortcuts[0].icons.len(), 1);
    assert_eq!(manifest.shortcuts[0].icons[0].src, Url::from_str("https://example.com/resources/shortcut.png").unwrap());
    assert!(manifest.shortcuts[0].icons[0].sizes.contains(&ImageSize::Fixed(32, 32)));

    assert_eq!(manifest.icons.len(), 2);
    assert_eq!(manifest.icons[0].src, Url::from_str("https://example.com/resources/icon1.png").unwrap());
    assert!(manifest.icons[0].purpose.contains(&ImagePurpose::Maskable));
    assert!(manifest.icons[0].purpose.contains(&ImagePurpose::Monochrome));
    assert_eq!(manifest.icons[0].label, Some("Example Icon".to_string()));
    assert_eq!(manifest.icons[1].src, Url::from_str("https://example.com/resources/icon2.png").unwrap());
    assert_eq!(manifest.icons[1].r#type, Some(MediaType::from_str("image/png").unwrap()));
    assert!(manifest.icons[1].sizes.contains(&ImageSize::Any));

    assert_eq!(manifest.screenshots.len(), 2);
    assert_eq!(manifest.screenshots[0].src, Url::from_str("https://example.com/resources/screenshot1.png").unwrap());
    assert_eq!(manifest.screenshots[0].platform, Some("windows".to_string()));
    assert_eq!(manifest.screenshots[0].label, Some("My Amazing App In Action".to_string()));
    assert_eq!(manifest.screenshots[1].src, Url::from_str("https://example.com/resources/screenshot2.png").unwrap());
    assert_eq!(manifest.screenshots[1].r#type, Some(MediaType::from_str("image/png").unwrap()));
    assert!(manifest.screenshots[1].sizes.contains(&ImageSize::Fixed(512, 512)));
}
