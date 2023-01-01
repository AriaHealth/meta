use sp_application_crypto::app_crypto;
use sp_application_crypto::sr25519;
use sp_core::crypto::KeyTypeId;
use sp_core::sr25519::Signature as Sr25519Signature;
use sp_runtime::traits::Verify;
use sp_runtime::MultiSignature;
use sp_runtime::MultiSigner;

pub const KEY_NAME: &[u8; 4] = b"mtrg";
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*KEY_NAME);

app_crypto!(sr25519, KEY_TYPE);

pub struct TestAuthId;

// implemented for runtime
impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
  type RuntimeAppPublic = Public;
  type GenericSignature = sp_core::sr25519::Signature;
  type GenericPublic = sp_core::sr25519::Public;
}

impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature> for TestAuthId {
  type RuntimeAppPublic = Public;
  type GenericSignature = sp_core::sr25519::Signature;
  type GenericPublic = sp_core::sr25519::Public;
}
