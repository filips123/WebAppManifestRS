#![allow(missing_docs)]

//! Contains all manifest errors.

use custom_error::custom_error;
use url::ParseError;

custom_error! {#[derive(PartialEq)] pub ManifestError
    /// When unknown URL is provided in invalid context (e.g., when not in `scope` or `start_url`) fields).
    InvalidUnknownUrl = "Provided unknown URL in invalid context",

    /// When URL parser encountered an error.
    UrlParsing {source: ParseError} = "Error while parsing URL ({source})",

    /// When two URLs are not in the same origin.
    NotSameOrigin {url1: String, url2: String} = "Provided URLs ({url1}, {url2}) are not in the same origin",

    /// When the URL is not within the scope.
    NotWithinScope {url: String, scope: String} = "Provided URL ({url}) is not within scope ({scope})",
}
