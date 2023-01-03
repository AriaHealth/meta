use crate::types::Registry;

// traits
pub trait CustodianRules<AccountId> {
  fn is_authorized(account: &AccountId) -> bool;
}

pub trait IssuerRules<AccountId> {
  fn can_create(owner_id: &AccountId, issuer_id: &AccountId, author_id: &AccountId) -> bool;

  fn on_create(registry: &Registry<AccountId>);

  fn can_update(old_registry: &Registry<AccountId>, new_registry: &Registry<AccountId>, author_id: &AccountId) -> bool;

  fn on_update(new_registry: &Registry<AccountId>, author_id: &AccountId);

  fn can_delete(registry: &Registry<AccountId>, author_id: &AccountId) -> bool;

  fn on_delete(registry: &Registry<AccountId>, author_id: &AccountId);
}

pub trait CombinedIdentifier<K1, K2, R> {
  fn compose(k1: &K1, k2: &K2) -> R;
  fn decompose(&self) -> (K1, K2);
}

// blank implementation

impl<AccountId> CustodianRules<AccountId> for () {
  #[allow(unused_variables)]
  fn is_authorized(account: &AccountId) -> bool {
    true
  }
}

impl<AccountId> IssuerRules<AccountId> for () {
  #[allow(unused_variables)]
  fn can_create(owner_id: &AccountId, issuer_id: &AccountId, author_id: &AccountId) -> bool {
    true
  }

  #[allow(unused_variables)]
  fn on_create(registry: &Registry<AccountId>) {
    // do nothing
  }

  #[allow(unused_variables)]
  fn can_update(old_registry: &Registry<AccountId>, new_registry: &Registry<AccountId>, author_id: &AccountId) -> bool {
    true
  }

  #[allow(unused_variables)]
  fn on_update(new_registry: &Registry<AccountId>, author_id: &AccountId) {
    // do nothing
  }

  #[allow(unused_variables)]
  fn can_delete(registry: &Registry<AccountId>, author_id: &AccountId) -> bool {
    true
  }

  #[allow(unused_variables)]
  fn on_delete(registry: &Registry<AccountId>, author_id: &AccountId) {
    // do nothing
  }
}
