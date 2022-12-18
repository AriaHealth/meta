use super::pallet::*;
use frame_support::ensure;

use region::{Country, Region, SubRegion};

use crate::{
    constants::MAX_CUSTODIANS,
    types::{AccessControl, AccountStatus, Group, GroupId, GroupInfo, Relation},
};

impl<T: Config> Pallet<T> {
    pub fn add_custodian(custodian_id: &T::AccountId) -> Result<(), Error<T>> {
        let maybe_custodian = Accounts::<T>::get(custodian_id);
        let mut custodians = Custodians::<T>::get();
        let index = custodians.iter().position(|x| *x == custodian_id.clone());

        ensure!(maybe_custodian.is_some(), Error::<T>::AccountNotExisted);
        ensure!(
            1 + custodians.len() <= MAX_CUSTODIANS,
            Error::<T>::TooManyCustodians
        );
        ensure!(index.is_none(), Error::<T>::CustodianAlreadyRegistered);

        custodians.push(custodian_id.clone());
        Custodians::<T>::put(custodians);

        Ok(())
    }

    pub fn remove_custodian(custodian_id: &T::AccountId) -> Result<(), Error<T>> {
        let maybe_custodian = Accounts::<T>::get(custodian_id);
        let mut custodians = Custodians::<T>::get();
        let index = custodians.iter().position(|x| *x == custodian_id.clone());

        ensure!(maybe_custodian.is_some(), Error::<T>::AccountNotExisted);
        ensure!(
            custodians.len().saturating_sub(1) >= 1,
            Error::<T>::TooFewCustodians
        );
        ensure!(index.is_some(), Error::<T>::CustodianNotRegistered);

        custodians.remove(index.unwrap());
        Custodians::<T>::put(custodians);

        Ok(())
    }

    pub fn connect(from_id: &T::AccountId, to_id: &T::AccountId) -> Result<(), Error<T>> {
        let maybe_to = Accounts::<T>::get(to_id);
        let maybe_connection = Connections::<T>::get(from_id, to_id);

        ensure!(maybe_to.is_some(), Error::<T>::AccountNotExisted);
        ensure!(
            maybe_to.unwrap().status == AccountStatus::Live,
            Error::<T>::AccountNotLive
        );
        ensure!(maybe_connection.is_none(), Error::<T>::AlreadyConnecting);

        Connections::<T>::insert(from_id, to_id, Relation::Pending);

        Ok(())
    }

    pub fn do_connect(from_id: &T::AccountId, to_id: &T::AccountId) -> Result<(), Error<T>> {
        let maybe_connection = Connections::<T>::get(from_id, to_id);

        ensure!(maybe_connection.is_some(), Error::<T>::NeverConnecting);
        ensure!(
            maybe_connection.unwrap() == Relation::Pending,
            Error::<T>::OnlyPendingAllowed
        );

        Connections::<T>::remove(from_id, to_id);
        Connections::<T>::insert(from_id, to_id, Relation::Connected);

        Ok(())
    }

    pub fn disconnect(from_id: &T::AccountId, to_id: &T::AccountId) -> Result<(), Error<T>> {
        let maybe_connection = Connections::<T>::get(from_id, to_id);

        ensure!(maybe_connection.is_some(), Error::<T>::NeverConnecting);

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
        let maybe_admin = Accounts::<T>::get(admin_id);
        let maybe_group = Groups::<T>::get(group_id);

        ensure!(maybe_group.is_none(), Error::<T>::GroupAlreadyExisted);
        ensure!(maybe_admin.is_some(), Error::<T>::AccountNotExisted);
        ensure!(
            maybe_admin.unwrap().status == AccountStatus::Live,
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
        let maybe_who = Accounts::<T>::get(who_id);
        let maybe_invoker = Accounts::<T>::get(invoker_id);
        let maybe_invoker_access = AccessControls::<T>::get(group_id, invoker_id);
        let maybe_group = Groups::<T>::get(group_id);

        ensure!(maybe_group.is_some(), Error::<T>::GroupNotExisted);
        ensure!(maybe_who.is_some(), Error::<T>::AccountNotExisted);
        ensure!(
            maybe_who.unwrap().status == AccountStatus::Live,
            Error::<T>::AccountNotLive
        );
        ensure!(maybe_invoker.is_some(), Error::<T>::AccountNotExisted);
        ensure!(
            maybe_invoker.unwrap().status == AccountStatus::Live,
            Error::<T>::AccountNotLive
        );
        ensure!(maybe_invoker_access.is_some(), Error::<T>::NeverJoining);
        ensure!(
            [AccessControl::SuperAdmin, AccessControl::Admin]
                .contains(&maybe_invoker_access.unwrap()),
            Error::<T>::OnlyAdminAllowed
        );

        Groups::<T>::try_mutate(group_id, |maybe_group| -> Result<(), Error<T>> {
            let mut group = maybe_group.take().ok_or(Error::<T>::GroupNotExisted)?;

            let members = group.members.checked_add(1).ok_or(Error::<T>::Overflow)?;
            group.members = members;

            if [AccessControl::SuperAdmin, AccessControl::Admin].contains(access_control) {
                let admins = group.admins.checked_add(1).ok_or(Error::<T>::Overflow)?;

                group.admins = admins;
            }

            *maybe_group = Some(group);

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
        let maybe_who = Accounts::<T>::get(who_id);
        let maybe_who_access = AccessControls::<T>::get(group_id, who_id);
        let maybe_invoker = Accounts::<T>::get(invoker_id);
        let maybe_invoker_access = AccessControls::<T>::get(group_id, invoker_id);
        let maybe_group = Groups::<T>::get(group_id);

        ensure!(maybe_group.is_some(), Error::<T>::GroupNotExisted);
        ensure!(maybe_who_access.is_some(), Error::<T>::NeverJoining);
        ensure!(maybe_who.is_some(), Error::<T>::AccountNotExisted);
        ensure!(
            maybe_who.unwrap().status == AccountStatus::Live,
            Error::<T>::AccountNotLive
        );
        ensure!(maybe_invoker.is_some(), Error::<T>::AccountNotExisted);
        ensure!(
            maybe_invoker.unwrap().status == AccountStatus::Live,
            Error::<T>::AccountNotLive
        );
        ensure!(maybe_invoker_access.is_some(), Error::<T>::NeverJoining);
        ensure!(
            [AccessControl::SuperAdmin, AccessControl::Admin]
                .contains(&maybe_invoker_access.unwrap()),
            Error::<T>::OnlyAdminAllowed
        );

        Groups::<T>::try_mutate(group_id, |maybe_group| -> Result<(), Error<T>> {
            let mut group = maybe_group.take().ok_or(Error::<T>::GroupNotExisted)?;

            let members = group.members.checked_sub(1).unwrap_or(0);
            group.members = members;

            if [AccessControl::SuperAdmin, AccessControl::Admin]
                .contains(&maybe_who_access.unwrap())
            {
                let admins = group.admins.checked_sub(1).unwrap_or(0);

                group.admins = admins;
            }

            *maybe_group = Some(group);

            Ok(())
        })?;
        AccessControls::<T>::remove(group_id, who_id);

        Ok(())
    }
}
