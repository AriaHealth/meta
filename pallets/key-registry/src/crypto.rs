use sp_core::crypto::KeyTypeId;
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"kyrg");

use sp_application_crypto::app_crypto;
use sp_application_crypto::sr25519;
use sp_core::sr25519::Signature as Sr25519Signature;
use sp_runtime::traits::Verify;
use sp_runtime::MultiSignature;
use sp_runtime::MultiSigner;

app_crypto!(sr25519, KEY_TYPE);

pub struct WorkerAuthorityId;

// implemented for runtime
impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for WorkerAuthorityId {
  type RuntimeAppPublic = Public;
  type GenericSignature = sp_core::sr25519::Signature;
  type GenericPublic = sp_core::sr25519::Public;
}

impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature> for WorkerAuthorityId {
  type RuntimeAppPublic = Public;
  type GenericSignature = sp_core::sr25519::Signature;
  type GenericPublic = sp_core::sr25519::Public;
}
