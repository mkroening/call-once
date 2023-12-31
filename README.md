# call-once

[![Crates.io](https://img.shields.io/crates/v/call-once)](https://crates.io/crates/call-once)
[![docs.rs](https://img.shields.io/docsrs/call-once)](https://docs.rs/call-once)
[![CI](https://github.com/mkroening/call-once/actions/workflows/ci.yml/badge.svg)](https://github.com/mkroening/call-once/actions/workflows/ci.yml)

This crate provides [`CallOnce`], a simple, thread-safe type that can only be called successfully _once_:

```rust
use call_once::CallOnce;

static CALL_ONCE: CallOnce = CallOnce::new();

assert!(CALL_ONCE.call_once().is_ok());
assert!(CALL_ONCE.call_once().is_err());
```

[`CallOnce`]: https://docs.rs/call-once/latest/call_once/struct.CallOnce.html

For API documentation, see the [docs].

[docs]: https://docs.rs/call-once

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
