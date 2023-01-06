use frame_system::Config;

pub trait ConnectionRulesProvider<T: Config> {
  type Error;

  fn can_connect(from: &T::AccountId, to: &T::AccountId) -> Result<bool, Self::Error>;

  fn on_connect(from: &T::AccountId, to: &T::AccountId) -> Result<(), Self::Error>;

  fn can_join_group(who: &T::AccountId, group_id: &T::Hash) -> Result<bool, Self::Error>;

  fn on_join_group(who: &T::AccountId, group_id: &T::Hash) -> Result<(), Self::Error>;
}

pub trait CustodianRulesProvider<T: Config> {
  type Error;

  fn is_authorized(account: &T::AccountId) -> Result<bool, Self::Error>;
}

pub trait IssuerRulesProvider<T: Config> {
  type Error;

  fn can_create(owner_id: &T::AccountId, issuer_id: &T::AccountId, author_id: &T::AccountId) -> Result<bool, Self::Error>;

  fn on_create(registry_id: &T::Hash) -> Result<(), Self::Error>;

  fn can_update(old_registry_id: &T::Hash, new_registry_id: &T::Hash, author_id: &T::AccountId) -> Result<bool, Self::Error>;

  fn on_update(new_registry_id: &T::Hash, author_id: &T::AccountId) -> Result<(), Self::Error>;

  fn can_delete(registry_id: &T::Hash, author_id: &T::AccountId) -> Result<bool, Self::Error>;

  fn on_delete(registry_id: &T::Hash, author_id: &T::AccountId) -> Result<(), Self::Error>;
}
