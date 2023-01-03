use super::pallet::*;
use crate::constants::MAX_CUSTODIANS;
use crate::types::AccountDetail;
use crate::types::AccountStatus;
use crate::types::Group;
use crate::types::GroupId;
use crate::types::GroupInfo;
use crate::types::Permission;
use crate::types::Relation;
use ap_region::Country;
use ap_region::Region;
use ap_region::SubRegion;
use frame_support::ensure;

impl<T: Config> Pallet<T> {
  pub fn do_add_custodian(issuer_id: &T::AccountId, custodian_id: &T::AccountId) -> Result<(), Error<T>> {
    let mut custodians = Custodians::<T>::get();
    let issuer_index = custodians.iter().position(|x| *x == issuer_id.clone());
    let custodian_index = custodians.iter().position(|x| *x == custodian_id.clone());

    ensure!(custodians.len() < MAX_CUSTODIANS, Error::<T>::TooManyCustodians);
    ensure!(custodian_index.is_none(), Error::<T>::CustodianAlreadyRegistered);
    ensure!(issuer_index.is_some(), Error::<T>::Unauthorized);

    custodians.push(custodian_id.clone());
    Custodians::<T>::put(custodians);

    if !Accounts::<T>::contains_key(custodian_id) {
      Accounts::<T>::insert(
        custodian_id.clone(),
        AccountDetail::<T::AccountId> {
          issuer: issuer_id.clone(),
          freezer: Default::default(),
          status: Default::default(),
          info: Default::default(),
        },
      );
    }

    Ok(())
  }

  pub fn do_remove_custodian(issuer_id: &T::AccountId, custodian_id: &T::AccountId) -> Result<(), Error<T>> {
    let mut custodians = Custodians::<T>::get();
    let issuer_index = custodians.iter().position(|x| *x == issuer_id.clone());
    let custodian_index = custodians.iter().position(|x| *x == custodian_id.clone());

    ensure!(custodians.len() > 2, Error::<T>::TooFewCustodians);
    ensure!(custodian_index.is_some(), Error::<T>::CustodianNotRegistered);
    ensure!(issuer_index.is_some(), Error::<T>::Unauthorized);
    ensure!(issuer_id.clone() != custodian_id.clone(), Error::<T>::SelfOperationNotAllowed);

    custodians.remove(custodian_index.unwrap());
    Custodians::<T>::put(custodians);

    Ok(())
  }

  pub fn do_connect(from_id: &T::AccountId, to_id: &T::AccountId) -> Result<(), Error<T>> {
    let maybe_to = Accounts::<T>::get(to_id);

    ensure!(maybe_to.is_some(), Error::<T>::AccountNotExisted);
    ensure!(maybe_to.unwrap().status == AccountStatus::Live, Error::<T>::AccountNotLive);

    let (k1, k2) = Self::sort_account_ids(from_id, to_id);
    if !Connections::<T>::contains_key(k1.clone(), k2.clone()) {
      Connections::<T>::insert(k1, k2, Relation::Connected);
    }

    Ok(())
  }

  pub fn do_disconnect(from_id: &T::AccountId, to_id: &T::AccountId) -> Result<(), Error<T>> {
    let (k1, k2) = Self::sort_account_ids(from_id, to_id);
    if Connections::<T>::contains_key(k1.clone(), k2.clone()) {
      Connections::<T>::remove(k1, k2);
    }

    Ok(())
  }

  pub fn do_create_group(
    custodian_id: &T::AccountId,
    admin_id: &T::AccountId,
    group_id: &GroupId,
    group_info: &GroupInfo,
    country: &Country,
    region: &Region,
    sub_region: &SubRegion,
  ) -> Result<(), Error<T>> {
    ensure!(Self::custodians().contains(custodian_id), Error::<T>::CustodianNotRegistered);
    ensure!(!Groups::<T>::contains_key(group_id), Error::<T>::GroupAlreadyExisted);

    Groups::<T>::insert(
      group_id,
      Group {
        admin_id: admin_id.clone(),
        issuer_id: custodian_id.clone(),
        info: group_info.clone(),
        country: country.clone(),
        region: region.clone(),
        sub_region: sub_region.clone(),
        admin_count: 1,
        member_count: 1,
      },
    );
    GroupPermissions::<T>::insert(group_id, admin_id, Permission::SuperAdmin);

    if !Accounts::<T>::contains_key(admin_id) {
      Accounts::<T>::insert(
        admin_id.clone(),
        AccountDetail::<T::AccountId> {
          issuer: custodian_id.clone(),
          freezer: Default::default(),
          status: AccountStatus::Live,
          info: Default::default(),
        },
      );
    }

    let (k1, k2) = Self::sort_account_ids(custodian_id, admin_id);
    if !Connections::<T>::contains_key(k1.clone(), k2.clone()) {
      Connections::<T>::insert(k1, k2, Relation::Connected);
    }

    Ok(())
  }

  pub fn do_join_group(issuer_id: &T::AccountId, joiner_id: &T::AccountId, group_id: &GroupId, permission: &Permission) -> Result<(), Error<T>> {
    let maybe_issuer = Accounts::<T>::get(issuer_id);
    let maybe_issuer_permission = GroupPermissions::<T>::get(group_id, issuer_id);

    ensure!(Groups::<T>::contains_key(group_id), Error::<T>::GroupNotExisted);
    ensure!(maybe_issuer.is_some(), Error::<T>::AccountNotExisted);
    ensure!(maybe_issuer.unwrap().status == AccountStatus::Live, Error::<T>::AccountNotLive);
    ensure!(maybe_issuer_permission.is_some(), Error::<T>::NoPermission);
    ensure!(
      [Permission::SuperAdmin, Permission::Admin].contains(&maybe_issuer_permission.unwrap()),
      Error::<T>::OnlyAdminAllowed
    );
    ensure!(issuer_id.clone() != joiner_id.clone(), Error::<T>::SelfOperationNotAllowed);

    Groups::<T>::try_mutate(group_id, |maybe_group| -> Result<(), Error<T>> {
      let mut group = maybe_group.take().ok_or(Error::<T>::GroupNotExisted)?;

      let members = group.member_count.checked_add(1).ok_or(Error::<T>::ValueOverflow)?;
      group.member_count = members;

      if [Permission::SuperAdmin, Permission::Admin].contains(permission) {
        let admins = group.admin_count.checked_add(1).ok_or(Error::<T>::ValueOverflow)?;

        group.admin_count = admins;
      }

      *maybe_group = Some(group);

      Ok(())
    })?;

    GroupPermissions::<T>::insert(group_id, joiner_id, permission);

    let (k1, k2) = Self::sort_account_ids(issuer_id, joiner_id);
    if !Connections::<T>::contains_key(k1.clone(), k2.clone()) {
      Connections::<T>::insert(k1, k2, Relation::Connected);
    }

    Ok(())
  }

  pub fn do_disjoin_group(issuer_id: &T::AccountId, disjoiner_id: &T::AccountId, group_id: &GroupId) -> Result<(), Error<T>> {
    let maybe_disjoiner = Accounts::<T>::get(disjoiner_id);
    let maybe_disjoiner_permission = GroupPermissions::<T>::get(group_id, disjoiner_id);
    let maybe_issuer = Accounts::<T>::get(issuer_id);
    let maybe_issuer_permission = GroupPermissions::<T>::get(group_id, issuer_id);

    ensure!(Groups::<T>::contains_key(group_id), Error::<T>::GroupNotExisted);
    ensure!(GroupPermissions::<T>::contains_key(group_id, disjoiner_id), Error::<T>::NeverJoined);
    ensure!(maybe_disjoiner.is_some(), Error::<T>::AccountNotExisted);
    ensure!(maybe_issuer.is_some(), Error::<T>::AccountNotExisted);
    ensure!(maybe_issuer.unwrap().status == AccountStatus::Live, Error::<T>::AccountNotLive);
    ensure!(maybe_issuer_permission.is_some(), Error::<T>::NoPermission);
    ensure!(
      [Permission::SuperAdmin, Permission::Admin].contains(&maybe_issuer_permission.unwrap()),
      Error::<T>::OnlyAdminAllowed
    );
    ensure!(issuer_id.clone() != disjoiner_id.clone(), Error::<T>::SelfOperationNotAllowed);

    Groups::<T>::try_mutate(group_id, |maybe_group| -> Result<(), Error<T>> {
      let mut group = maybe_group.take().ok_or(Error::<T>::GroupNotExisted)?;

      let members = group.member_count.saturating_sub(1);
      group.member_count = members;

      if [Permission::SuperAdmin, Permission::Admin].contains(&maybe_disjoiner_permission.unwrap()) {
        let admins = group.admin_count.saturating_sub(1);

        group.admin_count = admins;
      }

      *maybe_group = Some(group);

      Ok(())
    })?;
    GroupPermissions::<T>::remove(group_id, disjoiner_id);

    Ok(())
  }

  pub fn do_update_permission(
    issuer_id: &T::AccountId,
    member_id: &T::AccountId,
    group_id: &GroupId,
    permission: &Permission,
  ) -> Result<(), Error<T>> {
    let maybe_member = Accounts::<T>::get(member_id);
    let maybe_member_permission = GroupPermissions::<T>::get(group_id, member_id);
    let maybe_issuer = Accounts::<T>::get(issuer_id);
    let maybe_issuer_permission = GroupPermissions::<T>::get(group_id, issuer_id);

    ensure!(Groups::<T>::contains_key(group_id), Error::<T>::GroupNotExisted);
    ensure!(GroupPermissions::<T>::contains_key(group_id, member_id), Error::<T>::NeverJoined);
    ensure!(maybe_member.is_some(), Error::<T>::AccountNotExisted);
    ensure!(maybe_issuer.is_some(), Error::<T>::AccountNotExisted);
    ensure!(maybe_issuer.unwrap().status == AccountStatus::Live, Error::<T>::AccountNotLive);
    ensure!(maybe_issuer_permission.is_some(), Error::<T>::NoPermission);
    ensure!(
      [Permission::SuperAdmin, Permission::Admin].contains(&maybe_issuer_permission.unwrap()),
      Error::<T>::OnlyAdminAllowed
    );
    ensure!(issuer_id.clone() != member_id.clone(), Error::<T>::SelfOperationNotAllowed);

    if *permission == Permission::SuperAdmin {
      ensure!(
        maybe_issuer_permission.unwrap() == Permission::SuperAdmin,
        Error::<T>::OnlySuperAdminAllowed
      );
    }

    let member_permission = maybe_member_permission.unwrap();

    // The member is an admin, but the new permission is not admin
    if [Permission::SuperAdmin, Permission::Admin].contains(&member_permission) && ![Permission::SuperAdmin, Permission::Admin].contains(permission)
    {
      Groups::<T>::try_mutate(group_id, |maybe_group| -> Result<(), Error<T>> {
        let mut group = maybe_group.take().ok_or(Error::<T>::GroupNotExisted)?;

        let new_admin_count = group.admin_count.saturating_sub(1);

        group.admin_count = new_admin_count;

        *maybe_group = Some(group);

        Ok(())
      })?;
    }

    // The member is not an admin, but the new permission is admin
    if ![Permission::SuperAdmin, Permission::Admin].contains(&member_permission) && [Permission::SuperAdmin, Permission::Admin].contains(permission)
    {
      Groups::<T>::try_mutate(group_id, |maybe_group| -> Result<(), Error<T>> {
        let mut group = maybe_group.take().ok_or(Error::<T>::GroupNotExisted)?;

        let new_admin_count = group.admin_count.saturating_add(1);

        group.admin_count = new_admin_count;

        *maybe_group = Some(group);

        Ok(())
      })?;
    }

    GroupPermissions::<T>::remove(group_id, member_id);
    GroupPermissions::<T>::insert(group_id, member_id, *permission);

    Ok(())
  }

  fn sort_account_ids(k1: &T::AccountId, k2: &T::AccountId) -> (T::AccountId, T::AccountId) {
    if *k1 < *k2 {
      (k1.clone(), k2.clone())
    } else {
      (k2.clone(), k1.clone())
    }
  }
}
