use frame_support::dispatch::DispatchResultWithPostInfo;

pub trait ConnectionRules<AccountId> {
  fn can_connect(from: &AccountId, to: &AccountId) -> bool;

  fn on_connect(from: &AccountId, to: &AccountId) -> DispatchResultWithPostInfo;

  fn can_join_group(who: &AccountId, group_id: &[u8; 32]) -> bool;

  fn on_join_group(who: &AccountId, group_id: &[u8; 32]) -> DispatchResultWithPostInfo;
}

// blank implementation

impl<AccountId> ConnectionRules<AccountId> for () {
  #[allow(unused_variables)]
  fn can_connect(from: &AccountId, to: &AccountId) -> bool {
    true
  }

  #[allow(unused_variables)]
  fn on_connect(from: &AccountId, to: &AccountId) -> DispatchResultWithPostInfo {
    Ok(().into())
  }

  #[allow(unused_variables)]
  fn can_join_group(who: &AccountId, group_id: &[u8; 32]) -> bool {
    true
  }

  #[allow(unused_variables)]
  fn on_join_group(who: &AccountId, group_id: &[u8; 32]) -> DispatchResultWithPostInfo {
    Ok(().into())
  }
}
