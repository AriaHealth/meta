use super::pallet::*;
use frame_support::ensure;

use common::types::{Country, Region, SubRegion};

use crate::types::{AccessControl, AccountStatus, Group, GroupId, GroupInfo, Relation};

impl<T: Config> Pallet<T> {
    pub fn connect(from_id: &T::AccountId, to_id: &T::AccountId) -> Result<(), Error<T>> {
        let to = Accounts::<T>::get(to_id);
        let connection = Connections::<T>::get(from_id, to_id);

        ensure!(to.is_some(), Error::<T>::AccountNotExisted);
        ensure!(
            to.unwrap().status == AccountStatus::Live,
            Error::<T>::AccountNotLive
        );
        ensure!(connection.is_none(), Error::<T>::AlreadyConnecting);

        Connections::<T>::insert(from_id, to_id, Relation::Pending);

        Ok(())
    }

    pub fn do_connect(from_id: &T::AccountId, to_id: &T::AccountId) -> Result<(), Error<T>> {
        let connection = Connections::<T>::get(from_id, to_id);

        ensure!(connection.is_some(), Error::<T>::NeverConnecting);
        ensure!(
            connection.unwrap() == Relation::Pending,
            Error::<T>::OnlyPendingAllowed
        );

        Connections::<T>::remove(from_id, to_id);
        Connections::<T>::insert(from_id, to_id, Relation::Connected);

        Ok(())
    }

    pub fn disconnect(from_id: &T::AccountId, to_id: &T::AccountId) -> Result<(), Error<T>> {
        let connection = Connections::<T>::get(from_id, to_id);

        ensure!(connection.is_some(), Error::<T>::NeverConnecting);

        Connections::<T>::remove(from_id, to_id);

        Ok(())
    }

    pub fn create_group(
        admin_id: &T::AccountId,
        group_id: &GroupId,
        group_info: &GroupInfo,
        country: &Country,
        region: &Region,
        sub_region: &SubRegion,
    ) -> Result<(), Error<T>> {
        let admin = Accounts::<T>::get(admin_id);
        let group = Groups::<T>::get(group_id);

        ensure!(group.is_none(), Error::<T>::GroupAlreadyExisted);
        ensure!(admin.is_some(), Error::<T>::AccountNotExisted);
        ensure!(
            admin.unwrap().status == AccountStatus::Live,
            Error::<T>::AccountNotLive
        );

        Groups::<T>::insert(
            group_id,
            Group {
                owner: admin_id.clone(),
                info: group_info.clone(),
                country: country.clone(),
                region: region.clone(),
                sub_region: sub_region.clone(),
                admins: 1,
                members: 1,
            },
        );
        AccessControls::<T>::insert(group_id, admin_id, AccessControl::SuperAdmin);

        Ok(())
    }

    pub fn join(
        invoker_id: &T::AccountId,
        who_id: &T::AccountId,
        group_id: &GroupId,
        access_control: &AccessControl,
    ) -> Result<(), Error<T>> {
        let who = Accounts::<T>::get(who_id);
        let invoker = Accounts::<T>::get(invoker_id);
        let invoker_access = AccessControls::<T>::get(group_id, invoker_id);
        let group = Groups::<T>::get(group_id);

        ensure!(group.is_some(), Error::<T>::GroupNotExisted);
        ensure!(who.is_some(), Error::<T>::AccountNotExisted);
        ensure!(
            who.unwrap().status == AccountStatus::Live,
            Error::<T>::AccountNotLive
        );
        ensure!(invoker.is_some(), Error::<T>::AccountNotExisted);
        ensure!(
            invoker.unwrap().status == AccountStatus::Live,
            Error::<T>::AccountNotLive
        );
        ensure!(invoker_access.is_some(), Error::<T>::NeverJoining);
        ensure!(
            [AccessControl::SuperAdmin, AccessControl::Admin].contains(&invoker_access.unwrap()),
            Error::<T>::OnlyAdminAllowed
        );

        Groups::<T>::try_mutate(group_id, |maybe_details| -> Result<(), Error<T>> {
            let mut details = maybe_details.take().ok_or(Error::<T>::GroupNotExisted)?;

            let members = details.members.checked_add(1).ok_or(Error::<T>::Overflow)?;
            details.members = members;

            if [AccessControl::SuperAdmin, AccessControl::Admin].contains(access_control) {
                let admins = details.admins.checked_add(1).ok_or(Error::<T>::Overflow)?;

                details.admins = admins;
            }

            *maybe_details = Some(details);

            Ok(())
        })?;

        AccessControls::<T>::insert(group_id, who_id, access_control);

        Ok(())
    }

    pub fn disjoin(
        invoker_id: &T::AccountId,
        who_id: &T::AccountId,
        group_id: &GroupId,
    ) -> Result<(), Error<T>> {
        let who = Accounts::<T>::get(who_id);
        let who_access = AccessControls::<T>::get(group_id, who_id);
        let invoker = Accounts::<T>::get(invoker_id);
        let invoker_access = AccessControls::<T>::get(group_id, invoker_id);
        let group = Groups::<T>::get(group_id);

        ensure!(group.is_some(), Error::<T>::GroupNotExisted);
        ensure!(who_access.is_some(), Error::<T>::NeverJoining);
        ensure!(who.is_some(), Error::<T>::AccountNotExisted);
        ensure!(
            who.unwrap().status == AccountStatus::Live,
            Error::<T>::AccountNotLive
        );
        ensure!(invoker.is_some(), Error::<T>::AccountNotExisted);
        ensure!(
            invoker.unwrap().status == AccountStatus::Live,
            Error::<T>::AccountNotLive
        );
        ensure!(invoker_access.is_some(), Error::<T>::NeverJoining);
        ensure!(
            [AccessControl::SuperAdmin, AccessControl::Admin].contains(&invoker_access.unwrap()),
            Error::<T>::OnlyAdminAllowed
        );

        Groups::<T>::try_mutate(group_id, |maybe_details| -> Result<(), Error<T>> {
            let mut details = maybe_details.take().ok_or(Error::<T>::GroupNotExisted)?;

            let members = details.members.checked_sub(1).unwrap_or(0);
            details.members = members;

            if [AccessControl::SuperAdmin, AccessControl::Admin].contains(&who_access.unwrap()) {
                let admins = details.admins.checked_sub(1).unwrap_or(0);

                details.admins = admins;
            }

            *maybe_details = Some(details);

            Ok(())
        })?;
        AccessControls::<T>::remove(group_id, who_id);

        Ok(())
    }
}
