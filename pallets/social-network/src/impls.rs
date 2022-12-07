use crate::types::{AccessControl, GroupInfo, Relation};

use super::pallet::*;
use frame_support::{ensure, traits::Get};

impl<T: Config> Pallet<T> {
    pub fn connect(from: &T::AccountId, to: &T::AccountId) {
        Connections::<T>::insert(to, from, Relation::Pending);
    }

    pub fn do_connect(from: &T::AccountId, to: &T::AccountId) {
        Connections::<T>::insert(to, from, Relation::Connected);
    }

    pub fn disconnect(from: &T::AccountId, to: &T::AccountId) {
        Connections::<T>::remove(to, from);
    }

    pub fn create_group(group_admin: &T::AccountId, group_id: &T::GroupId, group_info: &GroupInfo) {
        Groups::<T>::insert(group_id, Some(group_info));
        GroupMembers::<T>::insert(group_id, group_admin, AccessControl::SuperAdmin);
    }

    pub fn delete_group(group_id: &T::GroupId) {
        // None group_info indicates that the group is deleted. If the deletion
        // reach the removal limit, the group will be deleted next time from the
        // on_idle hook.
        let _ = Groups::<T>::try_mutate(group_id, |group_info| {
            if let Some(group_info) = group_info {
                *group_info = None;
                Ok(())
            } else {
                Err(())
            }
        });
        GroupMembers::<T>::remove_prefix(group_id, Some(T::RemovalLimit::get()));
    }

    pub fn join(
        who: &T::AccountId,
        group_id: &T::GroupId,
        access_control: &AccessControl,
    ) -> Result<(), Error<T>> {
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
        let access_control = GroupMembers::<T>::get(group_id, who);

        ensure!(access_control.is_some(), Error::<T>::NeverJoining);
        ensure!(
            [
                AccessControl::PendingSuperAdmin,
                AccessControl::PendingAdmin,
                AccessControl::PendingReadOnly,
                AccessControl::PendingReadWrite,
                AccessControl::PendingMember
            ]
            .contains(&access_control.unwrap()),
            Error::<T>::AlreadyMember
        );

        match access_control {
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

    pub fn disjoin(who: &T::AccountId, group_id: &T::GroupId) {
        GroupMembers::<T>::remove(group_id, who);
    }
}
