#![deny(warnings, missing_docs)]
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(clippy))]

//! Safe password hashing. Adapted from
//! [secure-password](https://github.com/emilbayes/secure-password).
//!
//! ## Installation
//! ```sh
//! $ cargo add secure-password
//! ```

extern crate rust_sodium as sodium;

pub struct SecurePassword;

impl SecurePassword {
  pub fn hash() {}
  pub fn verify() {}
}
