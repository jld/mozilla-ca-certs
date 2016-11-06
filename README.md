# Mozilla CA Certs, as a Rust crate

[Documentation](https://docs.rs/mozilla-ca-certs/)

This crate statically embeds most of the information from the
[Mozilla CA Certificate Store][castore] into Rust programs.  This
includes all the certificates and raw trust entries, as well as more
limited sets of trust roots and explicit distrust items.

There's also an (optional, enabled by default) integration with
[webpki][webpki], which exposes the TLS server trust roots as webpki
`TrustAnchor`s.

[castore]: https://www.mozilla.org/en-US/about/governance/policies/security-group/certs/
[webpki]: https://briansmith.org/rustdoc/webpki/

## **Warning**

The Mozilla CA policy is mostly a set of trust roots, but there are
some more complicated cases, and they aren't all reflected properly in
this crate yet.  For example, the CNNIC roots: the official policy is
that it's untrusted except for a fixed list of 1,427 issued certs, but
this is reflected by marking the roots as completely trusted in
`certdata.txt` and expressing the restriction as
[C++ code in Firefox][cnnic-check].  Because this crate is currently
just a translation of `certdata.txt`, those roots are listed as
trusted when the truth is closer to the opposite of that.

[cnnic-check]: http://searchfox.org/mozilla-central/rev/cd1be634c9309c7fc99a3fde67dd44d343875f60/security/certverifier/NSSCertDBTrustDomain.cpp#753

## Related Work

The [webpki-roots][] crate also represents the Mozilla CA Certificate
Store in webpki form, but it makes its own policy decisions and
specifically excludes mostly-untrusted roots like CNNIC.  It doesn't
attempt to represent the distrusts from `certdata.txt`.

The build is also handled differently: webpki-roots includes
pre-generated source and a Python script for rebuilding it by
retrieving certificates from a Web service, whereas this crate
includes a copy of `certdata.txt` and parser/translates it at build time.

[webpki-roots]: https://github.com/ctz/webpki-roots

## Bugs

* Correctly represent Mozilla poicy with respect to semi-trusted
  certificates (see the Warning section, above).

* The Rust compiler is not very fast at handling large amounts of
  `[u8]` literals.  Build time could probably be improved by avoiding
  duplication in the source and/or allowing dependent crates to opt
  out of building the parts they're not using.

* Add some simple utilities for doing lookups in the distrust lists
  (and extend this to other forms of distrust, when implemented).
