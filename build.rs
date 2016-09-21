extern crate nss_certdata_parser;

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
}
