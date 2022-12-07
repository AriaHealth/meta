use super::pallet::*;
use frame_support::ensure;

use crate::types::{AccessControl, AccountStatus, GroupInfo, Relation};

impl<T: Config> Pallet<T> {
    pub fn connect(from: &T::AccountId, to: &T::AccountId) -> Result<(), Error<T>> {
        let account = Accounts::<T>::get(to);
        let connection = Connections::<T>::get(from, to);

        ensure!(account.is_some(), Error::<T>::AccountNotExisted);
        ensure!(
            account.unwrap().status == AccountStatus::Live,
            Error::<T>::AccountNotLive
        );
        ensure!(connection.is_none(), Error::<T>::AlreadyConnecting);

        Connections::<T>::insert(from, to, Relation::Pending);

        Ok(())
    }

    pub fn do_connect(from: &T::AccountId, to: &T::AccountId) -> Result<(), Error<T>> {
        let connection = Connections::<T>::get(from, to);

        ensure!(connection.is_some(), Error::<T>::NeverConnecting);
        ensure!(
            connection.unwrap() == Relation::Pending,
            Error::<T>::OnlyPendingAllowed
        );

        Connections::<T>::remove(from, to);
        Connections::<T>::insert(from, to, Relation::Connected);

        Ok(())
    }

    pub fn disconnect(from: &T::AccountId, to: &T::AccountId) -> Result<(), Error<T>> {
        let connection = Connections::<T>::get(from, to);

        ensure!(connection.is_some(), Error::<T>::NeverConnecting);

        Connections::<T>::remove(from, to);

        Ok(())
    }

    pub fn create_group(
        group_admin: &T::AccountId,
        group_id: &T::GroupId,
        group_info: &GroupInfo,
    ) -> Result<(), Error<T>> {
        let account = Accounts::<T>::get(group_admin);
        let group = Groups::<T>::get(group_id);

        ensure!(group.is_none(), Error::<T>::GroupAlreadyExisted);
        ensure!(account.is_some(), Error::<T>::AccountNotExisted);
        ensure!(
            account.unwrap().status == AccountStatus::Live,
            Error::<T>::AccountNotLive
        );

        Groups::<T>::insert(group_id, Some(group_info));
        GroupMembers::<T>::insert(group_id, group_admin, AccessControl::SuperAdmin);

        Ok(())
    }

    pub fn join(
        who: &T::AccountId,
        group_id: &T::GroupId,
        access_control: &AccessControl,
    ) -> Result<(), Error<T>> {
        let account = Accounts::<T>::get(who);
        let group = Groups::<T>::get(group_id);

        ensure!(group.is_some(), Error::<T>::GroupNotExisted);
        ensure!(account.is_some(), Error::<T>::AccountNotExisted);
        ensure!(
            account.unwrap().status == AccountStatus::Live,
            Error::<T>::AccountNotLive
        );
        ensure!(
            [
                AccessControl::PendingSuperAdmin,
                AccessControl::PendingAdmin,
                AccessControl::PendingReadOnly,
                AccessControl::PendingReadWrite,
                AccessControl::PendingMember
            ]
            .contains(access_control),
            Error::<T>::OnlyPendingAllowed
        );

        match access_control {
            AccessControl::PendingSuperAdmin => {
                GroupMembers::<T>::insert(group_id, who, AccessControl::SuperAdmin);
            }
            AccessControl::PendingAdmin => {
                GroupMembers::<T>::insert(group_id, who, AccessControl::Admin);
            }
            AccessControl::PendingReadOnly => {
                GroupMembers::<T>::insert(group_id, who, AccessControl::ReadOnly);
            }
            AccessControl::PendingReadWrite => {
                GroupMembers::<T>::insert(group_id, who, AccessControl::ReadWrite);
            }
            AccessControl::PendingMember => {
                GroupMembers::<T>::insert(group_id, who, AccessControl::Member);
            }
            _ => (),
        }

        Ok(())
    }

    pub fn do_join(who: &T::AccountId, group_id: &T::GroupId) -> Result<(), Error<T>> {
        let member = GroupMembers::<T>::get(group_id, who);
        let group = Groups::<T>::get(group_id);

        ensure!(group.is_some(), Error::<T>::GroupNotExisted);
        ensure!(member.is_some(), Error::<T>::NeverJoining);
        ensure!(
            [
                AccessControl::PendingSuperAdmin,
                AccessControl::PendingAdmin,
                AccessControl::PendingReadOnly,
                AccessControl::PendingReadWrite,
                AccessControl::PendingMember
            ]
            .contains(&member.unwrap()),
            Error::<T>::AlreadyJoined
        );

        match member {
            Some(AccessControl::PendingSuperAdmin) => {
                GroupMembers::<T>::remove(group_id, who);
                GroupMembers::<T>::insert(group_id, who, AccessControl::SuperAdmin);
            }
            Some(AccessControl::PendingAdmin) => {
                GroupMembers::<T>::remove(group_id, who);
                GroupMembers::<T>::insert(group_id, who, AccessControl::Admin);
            }
            Some(AccessControl::PendingReadOnly) => {
                GroupMembers::<T>::remove(group_id, who);
                GroupMembers::<T>::insert(group_id, who, AccessControl::ReadOnly);
            }
            Some(AccessControl::PendingReadWrite) => {
                GroupMembers::<T>::remove(group_id, who);
                GroupMembers::<T>::insert(group_id, who, AccessControl::ReadWrite);
            }
            Some(AccessControl::PendingMember) => {
                GroupMembers::<T>::remove(group_id, who);
                GroupMembers::<T>::insert(group_id, who, AccessControl::Member);
            }
            _ => (),
        }

        Ok(())
    }

    pub fn disjoin(who: &T::AccountId, group_id: &T::GroupId) -> Result<(), Error<T>> {
        let account = Accounts::<T>::get(who);
        let group = Groups::<T>::get(group_id);
        let member = GroupMembers::<T>::get(group_id, who);

        ensure!(group.is_some(), Error::<T>::GroupNotExisted);
        ensure!(member.is_some(), Error::<T>::NeverJoining);
        ensure!(account.is_some(), Error::<T>::AccountNotExisted);
        ensure!(
            account.unwrap().status == AccountStatus::Live,
            Error::<T>::AccountNotLive
        );

        GroupMembers::<T>::remove(group_id, who);

        Ok(())
    }
}
