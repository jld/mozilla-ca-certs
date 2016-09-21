pub mod types;
mod autogen {
    use types::*;
    use types::TrustLevel::*;
    // FIXME: is hard-coding the path separator like this portable?
    include!(concat!(env!("OUT_DIR"), "/certdata.rs"));
}

pub use types::{Certificate, Trust, TrustLevel};

/// (Documentation goes here.)
pub const ALL_CERTIFICATES: &'static [Certificate<'static>] =
    autogen::ALL_CERTIFICATES;

/// (Documentation goes here.)
pub const ALL_TRUST_RECORDS: &'static [Trust<'static>] =
    autogen::ALL_TRUST_RECORDS;

/// (Documentation goes here.)
pub const TLS_SERVER_TRUST_ROOTS: &'static [Certificate<'static>] =
    autogen::TLS_SERVER_TRUST_ROOTS;

/// (Documentation goes here.)
pub const TLS_SERVER_DISTRUSTS: &'static [Trust<'static>] =
    autogen::TLS_SERVER_DISTRUSTS;

/// (Documentation goes here.)
pub const EMAIL_TRUST_ROOTS: &'static [Certificate<'static>] =
    autogen::EMAIL_TRUST_ROOTS;

/// (Documentation goes here.)
pub const EMAIL_DISTRUSTS: &'static [Trust<'static>] =
    autogen::EMAIL_DISTRUSTS;

/// (Documentation goes here.)
pub const CODE_SIGNING_TRUST_ROOTS: &'static [Certificate<'static>] =
    autogen::CODE_SIGNING_TRUST_ROOTS;

/// (Documentation goes here.)
pub const CODE_SIGNING_DISTRUSTS: &'static [Trust<'static>] =
    autogen::CODE_SIGNING_DISTRUSTS;
