extern crate secure_password;

#[test]
fn hash_verify() {
  let pass = b"hello world";
  let hash = secure_password::hash(pass).unwrap();
  let is_ok = secure_password::verify(pass, &hash).unwrap();
  assert!(is_ok);
}
