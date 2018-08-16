# secure-password
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Safe password hashing with thread-local storage. Rust adaptation of
[emilbayes/secure-password](https://github.com/emilbayes/secure-password).

- [documentation][8]
- [crates.io][2]

## Security Disclaimer
:warning: This package has not been audited for security by any third party.
It uses [rust-argon2](https://docs.rs/rust-argon2) for the hash, and
[rand](https://docs.rs/rand) for the salt. Decide for yourself whether this
package is appropriate for the security profile of your project.
:warning:

## Installation
```sh
$ cargo add secure-password
```

## Usage
```rust
extern crate secure_password;

let pass = b"hello world";
let hash = secure_password::hash(pass).unwrap();
let is_ok = secure_password::verify(pass, &hash).unwrap();
assert!(is_ok);
```

## See Also
- [emilbayes/secure-password](https://github.com/emilbayes/secure-password).

## License
[Apache-2.0](./LICENSE)

[1]: https://img.shields.io/crates/v/secure-password.svg?style=flat-square
[2]: https://crates.io/crate/secure-password
[3]: https://img.shields.io/travis/yoshuawuyts/secure-password.svg?style=flat-square
[4]: https://travis-ci.org/yoshuawuyts/secure-password
[5]: https://img.shields.io/crates/d/secure-password.svg?style=flat-square
[6]: https://crates.io/crates/secure-password
[7]: https://docs.rs/secure-password/badge.svg
[8]: https://docs.rs/secure-password
