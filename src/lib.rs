#[cfg(feature = "use_webpki")]
extern crate webpki;

pub mod types;
mod autogen {
    use types::*;
    use types::TrustLevel::*;
    #[cfg(feature = "use_webpki")]
    use webpki::TrustAnchor;
    // FIXME: is hard-coding the path separator like this portable?
    include!(concat!(env!("OUT_DIR"), "/certdata.rs"));
}

pub use types::{Certificate, Trust, TrustLevel};

#[cfg(feature = "use_webpki")]
use webpki::TrustAnchor;

/// (Documentation goes here.)
pub static ALL_CERTIFICATES: &'static [Certificate<'static>] =
    autogen::ALL_CERTIFICATES;

/// (Documentation goes here.)
pub static ALL_TRUST_RECORDS: &'static [Trust<'static>] =
    autogen::ALL_TRUST_RECORDS;

/// (Documentation goes here.)
pub static TLS_SERVER_TRUST_ROOTS: &'static [Certificate<'static>] =
    autogen::TLS_SERVER_TRUST_ROOTS;

/// (Documentation goes here.)
pub static TLS_SERVER_DISTRUSTS: &'static [Trust<'static>] =
    autogen::TLS_SERVER_DISTRUSTS;

/// (Documentation goes here.)
pub static EMAIL_TRUST_ROOTS: &'static [Certificate<'static>] =
    autogen::EMAIL_TRUST_ROOTS;

/// (Documentation goes here.)
pub static EMAIL_DISTRUSTS: &'static [Trust<'static>] =
    autogen::EMAIL_DISTRUSTS;

/// (Documentation goes here.)
pub static CODE_SIGNING_TRUST_ROOTS: &'static [Certificate<'static>] =
    autogen::CODE_SIGNING_TRUST_ROOTS;

/// (Documentation goes here.)
pub static CODE_SIGNING_DISTRUSTS: &'static [Trust<'static>] =
    autogen::CODE_SIGNING_DISTRUSTS;

#[cfg(feature = "use_webpki")]
/// (Documentation goes here.)
pub static WEBPKI_TRUST_ROOTS: &'static [TrustAnchor<'static>] =
    &autogen::WEBPKI_TRUST_ROOTS;
