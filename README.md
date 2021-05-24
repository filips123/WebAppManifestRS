Web Application Manifest for Rust
=================================

[![version](https://img.shields.io/crates/v/web_app_manifest?label=version&style=flat-square)](https://crates.io/crates/web_app_manifest)
[![downloads](https://img.shields.io/crates/d/web_app_manifest?label=downloads&style=flat-square)](https://crates.io/crates/web_app_manifest)
[![license](https://img.shields.io/crates/l/web_app_manifest?label=license&style=flat-square)](https://github.com/filips123/WebAppManifestRS/blob/main/LICENSE)
[![docs](https://img.shields.io/docsrs/web_app_manifest?label=docs&style=flat-square)](https://docs.rs/web_app_manifest)
[![build](https://img.shields.io/github/workflow/status/filips123/WebAppManifestRS/checks?label=build&style=flat-square)](https://github.com/filips123/WebAppManifestRS/actions)

Rust data structure and utilities for (de)serialization and storage of Web Application Manifests.

## Description

This crate contains a [Serde-based][link-serde] data structure for parsing,
writing and storing [Web Application Manifests][link-mdn-manifest]. It complies
with the W3C specification and schema as much as possible and provides easy
access and type conversion for all official manifest fields. Because it uses
Serde, it supports much more than just JSON, so it can also be stored in more
efficient formats after being retrieved from the website.

## Warning

Due to reliance on some crates currently not published on Crates.io, this crate
can also not be published on Crates.io. You will need to include it through the
Git repository. When required crates are published on Crates.io, this crate will
also be published.

## Installation

You will need to install the crate from a Git repository. You will also need
to install `serde_json` crate for parsing JSON manifests, and optionally other
Serde-based crates for other storage formats.

Add this to your `Cargo.toml`:

```toml
[dependencies]
web_app_manifest = { git = "https://github.com/filips123/WebAppManifestRS" }
serde_json = "1.0"
```

**Note:** It is highly recommended to also specify `rev` and/or use
`Cargo.lock` to prevent breaking your project on backwards-incompatible
changes.

## Usage

### Parsing

To parse a manifest from JSON, use any `serde_json::from_` function,
depending on your data source. For example, to parse a manifest from
a string, use `serde_json::from_str`:

```rust
use web_app_manifest::WebAppManifest;

let json = r#"{
    "start_url": "/",
    "scope": "/",
    "name": "Example App",
    "short_name": "Example",
    "display": "standalone",
    "orientation": "portrait"
}"#;

let manifest: WebAppManifest = serde_json::from_str(json)?;
```

Because URLs can be relative and `serde_json` does not know how to resolve
them, you will also need to call the [`process`](WebAppManifest::process)
method and pas the document and manifest URLs to resolve all relative URLs
in the manifest and perform origin and scope validation:

```rust
use url::Url;

let document_url = Url::parse("https://example.com/index.html")?;
let manifest_url = Url::parse("https://example.com/site.webmanifest")?;

manifest.process(&document_url, &manifest_url)?;
```

You will now have the access to all manifest fields in the correct types:

```rust
use web_app_manifest::types::{Display, Orientation};

assert_eq!(manifest.name, Some("Example App".to_string()));
assert_eq!(manifest.short_name, Some("Example".to_string()));
assert_eq!(manifest.display, Display::Standalone);
assert_eq!(manifest.orientation, Orientation::Portrait);
```

### Creating

To create a new manifest, simply construct the struct:

```rust
use std::str::FromStr;

use web_app_manifest::WebAppManifest;
use web_app_manifest::types::Url;
use web_app_manifest::resources::IconResource;

let manifest = WebAppManifest {
    name: Some("Example App".to_string()),
    short_name: Some("Example".to_string()),

    start_url: Url::from_str("https://example.com/app/index.html")?,
    scope: Url::from_str("https://example.com/app")?,

    background_color: Some(csscolorparser::parse("rgb(100%,0%,0%)")?),
    theme_color: Some(csscolorparser::parse("aliceblue")?),

    icons: vec![IconResource {
        src: Url::from_str("/resources/icon.png")?,
        ..Default::default()
    }],

    ..Default::default()
};
```

**Important:** Always use `..Default::default()` when constructing
the manifest or its other structs. Adding new public fields will not
count as a major change, so your code could break without it.

Processing the manifest is not necessary, because it will be processed
when parsing by this crate or the browser in any case.

Then use any Serde-based function, such as `serde_json::to_string`,
to serialize it:

```rust
let json = serde_json::to_string(&manifest)?;
println!("{}", json);
```

### Other

See [docs][link-docs] of structs and fields for more documentation.
You can also check the [integration tests][link-tests] for usage examples.

## Versioning

This crate is currently in the alpha stage, expect backwards-incompatible
changes. Until it is published to Crates.io, there will be no Git tags or
official releases. Once it is ready to be published to Crates.io, it will
use a proper versioning scheme (SemVer).

**Note:** Adding new public struct fields or enum variants, if required by
the W3C specification updates, will not count as a major change. Your code
should always be using `..Default::default()` when constructing the manifest
or its other structs.

## Contributing

Any contribution intentionally submitted for inclusion in this repository
shall be dual-licensed under the MIT License and Apache License 2.0,
without any additional terms or conditions.

When changing the README file, edit the documentation in [`src/lib.rs`][link-lib-file]
instead, and use [`cargo-readme`][link-cargo-readme] to generate a README file.

Please also make sure that your code is properly linted and formatted using
[clippy][link-clippy] and [rustfmt][link-rustfmt].

## License

This library is licensed under the MIT License or Apache License 2.0, at
your opinion. See the [LICENSE][link-license-file] file for more details.

The documentation for manifest fields and enum variants is heavily based on
the documentation by MDN Web Docs and the W3C and WHATWG specifications,
licensed under the following licenses:

- [Web App Manifests on MDN][link-mdn-manifest] by Mozilla Contributors is licensed under the [CC-BY-SA-2.5][link-license-cc-by-sa-2.5].
- [Web Application Manifest (W3C Editor's Draft)][link-w3c-manifest] by W3C and Editors is licensed under the [W3C Document License][link-license-w3c].
- [Web Application Manifest - Application Info (W3C Editor's Draft)][link-w3c-manifest-app-info] by W3C and Editors is licensed under the [W3C Document License][link-license-w3c].
- [The Screen Orientation API (W3C Working Draft)][link-w3c-orientation] by W3C and Editors is licensed under the [W3C Document License][link-license-w3c].
- [HTML Living Standard][link-whatwg-html] by WHATWG and Editors is licensed under the [CC-BY-4.0][link-license-cc-by-4.0].

**Note:** The documentation and library are not affiliated with MDN or W3C in any way.
Any content in this documentation may not be the correct or latest W3C specification.
Read the official W3C specification for the accurate data.

[link-serde]: https://serde.rs/

[link-mdn-manifest]: https://developer.mozilla.org/en-US/docs/Web/Manifest
[link-w3c-manifest]: https://w3c.github.io/manifest/
[link-w3c-manifest-app-info]: https://w3c.github.io/manifest-app-info/
[link-w3c-orientation]: https://www.w3.org/TR/screen-orientation/
[link-whatwg-html]: https://html.spec.whatwg.org/multipage/semantics.html

[link-docs]: https://docs.rs/web_app_manifest
[link-tests]: https://github.com/filips123/WebAppManifestRS/blob/main/tests/tests.rs

[link-lib-file]: https://github.com/filips123/WebAppManifestRS/blob/main/src/lib.rs
[link-license-file]: https://github.com/filips123/WebAppManifestRS/blob/main/LICENSE

[link-license-cc-by-sa-2.5]: https://creativecommons.org/licenses/by-sa/2.5/
[link-license-cc-by-4.0]: https://creativecommons.org/licenses/by/4.0/
[link-license-w3c]: https://www.w3.org/Consortium/Legal/2015/doc-license

[link-cargo-readme]: https://github.com/livioribeiro/cargo-readme
[link-clippy]: https://github.com/rust-lang/rust-clippy
[link-rustfmt]: https://github.com/rust-lang/rustfmt
