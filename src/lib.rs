#![deny(warnings, missing_docs)]
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(clippy))]

//! Safe password hashing. Adapted from
//! [secure-password](https://github.com/emilbayes/secure-password).
//!
//! - [documentation]
//! - [crates.io]
//!
//! ## Features
//! - ~~State of the art password hashing algorithm (Argon2id)~~
//! - Safe defaults for most applications
//! - Future-proof so work factors and hashing algorithms can be easily upgraded
//!
//! ## Known Limitations
//! - There are no rust bindings to `Argon2id` yet, so we're currently using
//! `scryptsalsa208sha256`.
//! - We're going to have to release a breaking change once we move over, which
//! will require rehashing all passwords. Be warned!
//!
//! ## Usage
//!
//! ```rust,ignore
//! extern crate secure_password;
//!
//! use secure_password::SecurePassword;
//!
//! let pwd = SecurePassword::default();
//!
//! let user_password = b"Correct Horse Battery Staple";
//! let hash = pwd.hash(user_password)?;
//! // Save "hash" somewhere.
//!
//! let res = pwd.verify(user_password, hash)?;
//! if let secure_password::VerifyStatus::needs_rehash = res {
//!   if let Ok(improved_hash) pwd.hash(user_password) {
//!     println!("Yay, password successfully upgraded");
//!     // Save improved_hash somewhere
//!   }
//! }
//! ```
//!
//! ## Installation
//! ```sh
//! $ cargo add secure-password
//! ```
//!
//! [documentation]: https://docs.rs/secure-password
//! [crates.io]: https://crates.io/crates/secure-password

extern crate rust_sodium as sodium;

use sodium::crypto::pwhash;

use std::fmt;

/// A struct which exposes functions to hash and verify passwords.
#[derive(Debug)]
pub struct SecurePassword {
  memlimit_min: usize,
  memlimit_max: usize,
  opslimit_min: usize,
  opslimit_max: usize,
}

/// Represents the status of a successful call to `.verify()`.
pub enum VerifyStatus {
  /// The operation was successful.
  Ok,

  /// The operation was successful, but the password needs to be hashed again &
  /// stored to improve security.
  NeedsRehash,
}

impl SecurePassword {
  /// Hash a password.
  pub fn hash() {}

  /// Verify a hashed password.
  pub fn verify() {}
}

impl Default for SecurePassword {
  fn default() -> SecurePassword {
    SecurePassword {
      memlimit_min: 0,
      memlimit_max: 0,
      opslimit_min: 0,
      opslimit_max: 0,
    }
  }
}

impl fmt::Display for SecurePassword {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "")
  }
}
