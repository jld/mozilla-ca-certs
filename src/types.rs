/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

pub type Blob = [u8];
pub type Asn1 = Blob;

#[derive(Debug, Clone)]
pub struct Certificate<'a> {
    pub label: &'a str,
    pub cert: &'a Asn1,
    pub issuer: &'a Asn1,
    pub serial: &'a Asn1,
    pub subject: &'a Asn1,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrustLevel {
    Distrust,
    MustVerify,
    TrustedDelegator,
}

#[derive(Debug, Clone)]
pub struct Trust<'a> {
    // TODO: factor out these three fields, for list-of-distrusts use cases?
    pub label: &'a str,
    pub issuer: &'a Asn1,
    pub serial: &'a Asn1,
    pub tls_server_trust: TrustLevel,
    pub email_trust: TrustLevel,
    pub code_signing_trust: TrustLevel,
    // See misgivings in `nss_certdata_parser::structured::Trust`.
    pub md5: Option<&'a Blob>,
    pub sha1: Option<&'a Blob>,
}
