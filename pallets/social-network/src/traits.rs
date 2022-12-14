use crate::Config;
use frame_support::dispatch::DispatchResultWithPostInfo;

pub trait ConnectionRules<AccountId> {
  /// Check if `from` can connect to `to`. If `false`,the connection will be rejected.
  /// If `true`, the connection will be accepted. If the connection is accepted, `on_connect`
  /// will be called. If the connection is rejected, `on_connect` will not be called. If
  /// `can_connect` returns `false`, `on_connect` will not be called. If `can_connect` returns
  /// `true`, `on_connect` will be called.
  fn can_connect(from: &AccountId, to: &AccountId) -> bool;

  /// Called when `from` connects to `to`. If the connection is accepted, `on_connect` will be
  /// called. If the connection is rejected, `on_connect` will not be called. If `can_connect`
  /// returns `false`, `on_connect` will not be called. If `can_connect` returns `true`,
  /// `on_connect` will be called.
  fn on_connect(from: &AccountId, to: &AccountId) -> DispatchResultWithPostInfo;

  /// Check if `who` can join `group_id`. If `false`, the join will be rejected. If `true`,
  /// the join will be accepted. If the join is accepted, `on_join_group` will be called.
  /// If the join is rejected, `on_join_group` will not be called. If `can_join_group`
  /// returns `false`, `on_join_group` will not be called. If `can_join_group` returns `true`,
  /// `on_join_group` will be called.
  fn can_join_group(who: &AccountId, group_id: &[u8; 32]) -> bool;

  /// Called when `who` joins `group_id`. If the join is accepted, `on_join_group` will be
  /// called. If the join is rejected, `on_join_group` will not be called. If `can_join_group`
  /// returns `false`, `on_join_group` will not be called. If `can_join_group` returns `true`,
  /// `on_join_group` will be called.
  fn on_join_group(who: &AccountId, group_id: &[u8; 32]) -> DispatchResultWithPostInfo;
}

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
