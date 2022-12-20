use crate::constants::KEY_TYPE;
use sp_core::sr25519::Signature as Sr25519Signature;
use sp_runtime::{
  app_crypto::{app_crypto, sr25519},
  traits::Verify,
  MultiSignature, MultiSigner,
};
use sp_std::prelude::*;

app_crypto!(sr25519, KEY_TYPE);

pub struct AuthorityId;
// implemented for runtime
impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for AuthorityId {
  type RuntimeAppPublic = Public;
  type GenericSignature = sp_core::sr25519::Signature;
  type GenericPublic = sp_core::sr25519::Public;
}

// implemented for mock runtime in test
impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature> for AuthorityId {
  type RuntimeAppPublic = Public;
  type GenericSignature = sp_core::sr25519::Signature;
  type GenericPublic = sp_core::sr25519::Public;
}
