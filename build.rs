/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

extern crate nss_certdata_parser;
#[cfg(feature = "use_webpki")]
extern crate webpki;
#[cfg(feature = "use_webpki")]
extern crate untrusted;

use std::fs::File;
use std::env;
use std::io::{BufReader, Write};
use std::path::Path;

use nss_certdata_parser::{ObjectIter, CertData, Usage};

const CERTDATA_PATH: &'static str = "data/certdata.txt";
const CERTS_TY: &'static str = "&'static [Certificate<'static>]";
const TRUSTS_TY: &'static str = "&'static [Trust<'static>]";

macro_rules! gen {
    ($out:expr, $name:ident: $ty:ident = $val:expr) => {
        (writeln!($out, "pub const {}: {} = &{:#?};", stringify!($name), $ty, $val).unwrap())
    }
}

#[cfg(feature = "use_webpki")]
fn do_webpki<W: Write>(mut out: W, data: &CertData) {
    use std::fmt;
    use untrusted::Input;
    use webpki;
    use webpki::trust_anchor_util;

    const ANCHORS_TY: &'static str = "&'static [TrustAnchor<'static>]";

    // FIXME: fix this upstream.
    #[derive(Debug)]
    struct TrustAnchor<'a> {
        subject: Blob<'a>,
        spki: Blob<'a>,
        name_constraints: Option<Blob<'a>>,
    }
    struct Blob<'a>(&'a [u8]);
    impl<'a> fmt::Debug for Blob<'a> {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            write!(fmt, "&{:?}", self.0)
        }
    }

    let anchors: Vec<_> = data.trusted_certs(Usage::TlsServer).iter().map(|cert| {
        let webpki::TrustAnchor { subject, spki, name_constraints } =
            trust_anchor_util::cert_der_as_trust_anchor(Input::from(&cert.cert)).unwrap();
        TrustAnchor {
            subject: Blob(subject),
            spki: Blob(spki),
            name_constraints: name_constraints.map(Blob),
        }
    }).collect();

    gen!(out, WEBPKI_TRUST_ROOTS: ANCHORS_TY = anchors);
}

#[cfg(not(feature = "use_webpki"))]
fn do_webpki<W: Write>(_out: W, _data: &CertData) { }

fn main() {
    // FIXME -- does Cargo not like relative paths?
    // println!("cargo:rerun-if-changed:{}", CERTDATA_PATH);
    let out_dir = env::var("OUT_DIR").expect("$OUT_DIR not set");
    let objs = ObjectIter::new(BufReader::new(File::open(CERTDATA_PATH).unwrap()));
    let data = CertData::from_iter(objs).unwrap();
    let mut out = File::create(Path::new(&out_dir).join("certdata.rs")).unwrap();

    gen!(out, ALL_CERTIFICATES: CERTS_TY = data.certs());
    gen!(out, ALL_TRUST_RECORDS: TRUSTS_TY = data.trusts());
    gen!(out, TLS_SERVER_TRUST_ROOTS: CERTS_TY = data.trusted_certs(Usage::TlsServer));
    gen!(out, TLS_SERVER_DISTRUSTS: TRUSTS_TY = data.distrusts(Usage::TlsServer));
    gen!(out, EMAIL_TRUST_ROOTS: CERTS_TY = data.trusted_certs(Usage::Email));
    gen!(out, EMAIL_DISTRUSTS: TRUSTS_TY = data.distrusts(Usage::Email));
    gen!(out, CODE_SIGNING_TRUST_ROOTS: CERTS_TY = data.trusted_certs(Usage::CodeSigning));
    gen!(out, CODE_SIGNING_DISTRUSTS: TRUSTS_TY = data.distrusts(Usage::CodeSigning));
    do_webpki(&mut out, &data);
}
