use super::pallet::*;

#[pallet::call]
impl<T: Config> Pallet<T> {
    #[pallet::weight(10_000)]
    pub fn register_account(origin: OriginFor<T>, id: T::AccountId) {
        let creator_id = ensure_signed(origin)?;
    }
}
