use crate::types::{GroupId, GroupInfo, Relation};

use super::pallet::*;
use frame_support::traits::Get;

impl<T: Config> Pallet<T> {
    pub fn connect(from: &T::AccountId, to: &T::AccountId, relation: Relation) {
        Connections::<T>::insert(to, from, relation);
    }

    pub fn disconnect(from: &T::AccountId, to: &T::AccountId) {
        Connections::<T>::remove(to, from);
    }

    pub fn create_group(group_admin: &T::AccountId, group_id: &GroupId, group_info: &GroupInfo) {
        Groups::<T>::insert(group_id, Some(group_info));
        GroupAdmins::<T>::insert(group_id, group_admin, true);
    }

    pub fn delete_group(group_id: &GroupId) {
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
        GroupAdmins::<T>::remove_prefix(group_id, Some(T::RemovalLimit::get()));
        GroupMembers::<T>::remove_prefix(group_id, Some(T::RemovalLimit::get()));
    }

    pub fn join(who: &T::AccountId, group_id: &GroupId) {
        GroupMembers::<T>::insert(group_id, who, true);
    }

    pub fn disjoin(who: &T::AccountId, group_id: &GroupId) {
        GroupMembers::<T>::remove(group_id, who);
    }
}
