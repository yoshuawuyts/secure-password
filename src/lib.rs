#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

extern crate argon2;
extern crate failure;
extern crate rand;

use argon2::{Config, ThreadMode, Variant, Version};
use failure::Error;
use rand::prelude::*;

// use std::convert::AsRef;
use std::borrow::Borrow;
use std::rc::Rc;
use std::str;

thread_local! {
  static CONFIG: Rc<Config<'static>> = Rc::new(Config {
    variant: Variant::Argon2id,
    version: Version::Version13,
    mem_cost: 65536,
    time_cost: 1,
    lanes: 1,
    thread_mode: ThreadMode::Sequential,
    secret: &[],
    ad: &[],
    hash_length: 32,
  });
}

/// Hash a password.
pub fn hash(password: &[u8]) -> Result<Vec<u8>, Error> {
  let salt = random_bytes();
  let res = CONFIG.with(|config| {
    let config = config.borrow();
    argon2::hash_encoded(password, &salt, &config)
  })?;
  Ok(res.into_bytes())
}

/// Verify a password against a hash.
pub fn verify(password: &[u8], hash: &[u8]) -> Result<bool, Error> {
  let hash = str::from_utf8(hash)?;
  let matches = argon2::verify_encoded(&hash, password)?;
  Ok(matches)
}

fn random_bytes() -> Vec<u8> {
  let mut rng = rand::thread_rng();
  let mut res = vec![0; 256];
  rng.fill_bytes(&mut res);
  res
}
