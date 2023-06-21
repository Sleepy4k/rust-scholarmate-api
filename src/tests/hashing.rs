use pretty_assertions::assert_eq;

use crate::helpers::auth::*;

#[test]
fn test_hash_password() {
  let data = "password";
  let result = hash_password(data);

  assert_eq!(result, String::from("$argon2i$v=19$m=4096,t=3,p=1$bWVybW9hdXRoaGFzaA$NdWVPYd66iN7iKMoOiq2ANY1dtOAXVvklAJniaeYlWA"));
}

#[test]
fn test_hash_password_wrong_password() {
  let data = "password123";
  let result = hash_password(data);

  assert_ne!(result, String::from("$argon2i$v=19$m=4096,t=3,p=1$bWVybW9hdXRoaGFzaA$NdWVPYd66iN7iKMoOiq2ANY1dtOAXVvklAJniaeYlWA"));
}

#[test]
fn test_verify_password() {
  let data = "password";
  let hashed_password = "$argon2i$v=19$m=4096,t=3,p=1$bWVybW9hdXRoaGFzaA$NdWVPYd66iN7iKMoOiq2ANY1dtOAXVvklAJniaeYlWA";
  let result = verify_password(data, hashed_password);

  assert_eq!(result, true);
}

#[test]
fn test_verify_password_wrong_password() {
  let data = "password123";
  let hashed_password = "$argon2i$v=19$m=4096,t=3,p=1$bWVybW9hdXRoaGFzaA$NdWVPYd66iN7iKMoOiq2ANY1dtOAXVvklAJniaeYlWA";
  let result = verify_password(data, hashed_password);

  assert_eq!(result, false);
}
