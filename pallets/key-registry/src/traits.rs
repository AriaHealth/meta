use crate::types::Key;
use crate::types::KeyName;
use crate::types::KeyType;

pub trait KeyRules<AccountId> {
  fn can_create_key(issuer_id: &AccountId, key_type: &KeyType, key_name: &KeyName) -> bool;

  fn on_create_key(issuer_id: &AccountId, key_type: &KeyType, key_name: &KeyName);

  fn can_submit_registry(issuer_id: &AccountId, owner_id: &AccountId, key_type: &KeyType, key: &Key) -> bool;

  fn on_submit_registry(issuer_id: &AccountId, owner_id: &AccountId, key_type: &KeyType, key: &Key);
}

// blank implementation
impl<AccountId> KeyRules<AccountId> for () {
  #[allow(unused_variables)]
  fn can_create_key(issuer_id: &AccountId, key_type: &KeyType, key_name: &KeyName) -> bool {
    true
  }

  #[allow(unused_variables)]
  fn on_create_key(issuer_id: &AccountId, key_type: &KeyType, key_name: &KeyName) {
    // do nothing
  }

  #[allow(unused_variables)]
  fn can_submit_registry(issuer_id: &AccountId, owner_id: &AccountId, key_type: &KeyType, key: &Key) -> bool {
    true
  }

  #[allow(unused_variables)]
  fn on_submit_registry(issuer_id: &AccountId, owner_id: &AccountId, key_type: &KeyType, key: &Key) {
    // do nothing
  }
}
