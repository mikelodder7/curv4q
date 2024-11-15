# Curve Four $\mathbb{Q}$

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Apache2/MIT licensed][license-image]
[![Downloads][downloads-image]][crate-link]
![build](https://github.com/mikelodder7/curv4q/actions/workflows/curv4q.yml/badge.svg)
![MSRV][msrv-image]

![](./img/fourq.webp)

A rust implementation around the FourQLib library for the FourQ elliptic curve.

This crate provides the curve scalar and point operations for the FourQ elliptic curve,
SchnorrQ<sup><a href="#schnorrq">3</a></sup> signatures, and ECDH<sup><a href="#ecdh">4</a></sup> key exchange.

This library uses optimizations for 
- 32-bit x86 and ARM
- 64-bit x64, x86 and ARM and ARM64-NEON

and are auto-detected at compile time.

If targeting wasm then generic 32-bit operations are used.

## ⚠️ Security Warning

The implementation contained in this crate has never been independently audited!

USE AT YOUR OWN RISK!

## Minimum Supported Rust Version

This crate requires **Rust 1.82** at a minimum.

We may change the MSRV in the future, but it will be accompanied by a minor
version bump.

## License

Licensed under

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT license](http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

## References

- [FourQ: Four-Dimensional Decompositions on a Q-curve over the Mersenne Prime](https://eprint.iacr.org/2015/565.pdf)
- [IETF Draft](https://datatracker.ietf.org/doc/html/draft-ladd-cfrg-4q-01)
- [SchorrQ](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/07/SchnorrQ.pdf)<div id="schnorrq"></div>
- [ECDH](https://datatracker.ietf.org/doc/html/draft-ladd-cfrg-4q-01#section-5)<div id="ecdh"></div>
- [Real World Crypto 2017 Presentation](https://rwc.iacr.org/2017/Slides/patrick.longa.pdf)
- [FourQ on embedded devices with strong countermeasures against side-channel attacks](https://www.iacr.org/archive/ches2017/10529146/10529146.pdf)
- [FourQNEON: Faster Elliptic Curve Scalar Multiplications on ARM Processors](https://eprint.iacr.org/2016/645.pdf)

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/curv4q.svg
[crate-link]: https://crates.io/crates/curv4q
[docs-image]: https://docs.rs/curv4q/badge.svg
[docs-link]: https://docs.rs/curv4q/
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[downloads-image]: https://img.shields.io/crates/d/curv4q.svg
[msrv-image]: https://img.shields.io/badge/rustc-1.82+-blue.svg