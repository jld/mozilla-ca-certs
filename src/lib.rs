/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

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

/// All the certificates in the Mozilla CA Certificate Store, sorted
/// by `subject`.  This includes both legitimate CAs and known-bad
/// (compromised, fraudulent, etc.)  certificates that are explicitly
/// distrusted; the trust records are needed to tell which is which.
pub static ALL_CERTIFICATES: &'static [Certificate<'static>] =
    autogen::ALL_CERTIFICATES;

/// All of the trust records from the `certdata.txt` file, sorted by
/// `(issuer, serial)` to allow efficient searching given a certificate.
pub static ALL_TRUST_RECORDS: &'static [Trust<'static>] =
    autogen::ALL_TRUST_RECORDS;

/// All of the certificates declared as trusted delegators (trust
/// roots) for authenticating a TLS server.  Sorted by `subject` to
/// allow efficient searching given the issuer of a presented
/// certificate.
pub static TLS_SERVER_TRUST_ROOTS: &'static [Certificate<'static>] =
    autogen::TLS_SERVER_TRUST_ROOTS;

/// Trust records indicating certificates to be distrusted (overriding
/// any otherwise valid signatures) for authenticating a TLS server.
/// Sorted by `(issuer, serial)`, like `ALL_TRUST_RECORDS`.
pub static TLS_SERVER_DISTRUSTS: &'static [Trust<'static>] =
    autogen::TLS_SERVER_DISTRUSTS;

/// All of the certificates declared as trusted delegators (trust
/// roots) for use with email protection.  Sorted by `subject` to
/// allow efficient searching given the issuer of a presented
/// certificate.
pub static EMAIL_TRUST_ROOTS: &'static [Certificate<'static>] =
    autogen::EMAIL_TRUST_ROOTS;

/// Trust records indicating certificates to be distrusted (overriding
/// any otherwise valid signatures) for email protection.
/// Sorted by `(issuer, serial)`, like `ALL_TRUST_RECORDS`.
pub static EMAIL_DISTRUSTS: &'static [Trust<'static>] =
    autogen::EMAIL_DISTRUSTS;

/// All of the certificates declared as trusted delegators (trust
/// roots) for code signing.  Sorted by `subject` to
/// allow efficient searching given the issuer of a presented
/// certificate.
pub static CODE_SIGNING_TRUST_ROOTS: &'static [Certificate<'static>] =
    autogen::CODE_SIGNING_TRUST_ROOTS;

/// Trust records indicating certificates to be distrusted (overriding
/// any otherwise valid signatures) for code signing.
/// Sorted by `(issuer, serial)`, like `ALL_TRUST_RECORDS`.
pub static CODE_SIGNING_DISTRUSTS: &'static [Trust<'static>] =
    autogen::CODE_SIGNING_DISTRUSTS;

#[cfg(feature = "use_webpki")]
/// The same certificates as `TLS_SERVER_TRUST_ROOTS`, but pre-parsed
/// into `webpki::TrustAnchor` format.
pub static WEBPKI_TRUST_ROOTS: &'static [TrustAnchor<'static>] =
    &autogen::WEBPKI_TRUST_ROOTS;
