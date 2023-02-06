#![allow(missing_docs)]

//! Contains all manifest errors.

use thiserror::Error;

/// A manifest error represents all errors that can occur during manifest processing.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum ManifestError {
    /// When the URL parser encountered an error.
    #[error("Error while parsing the URL: {source}")]
    UrlParsing {
        #[from]
        source: url::ParseError,
    },

    /// When unknown URL is provided in invalid context (e.g., when not in `scope` or `start_url` fields).
    #[error("Provided unknown URL in invalid context")]
    InvalidUnknownUrl,

    /// When two URLs are not in the same origin.
    #[error("Provided URLs ({url1}, {url2}) are not in the same origin")]
    NotSameOrigin { url1: url::Url, url2: url::Url },

    /// When the URL is not within the scope.
    #[error("Provided URL ({url}) is not within the scope ({scope})")]
    NotWithinScope { url: url::Url, scope: url::Url },

    /// When the URL cannot be converted to an absolute URL.
    #[error("Provided URL cannot be converted to an absolute URL")]
    NotAbsolute { url: crate::types::Url },

    /// When the URL cannot be converted to `String`.
    #[error("Provided URL cannot be converted to `String`")]
    NotStringifyable { url: crate::types::Url },
}
