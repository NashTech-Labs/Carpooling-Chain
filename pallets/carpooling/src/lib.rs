#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    // CustomerOf is datatype used for storage in Customer
    type CustomerOf<T> = SCustomer<<T as frame_system::Config>::Hash>;
    // SCustomer is a struct for Customer
    #[derive(Encode, Decode, Copy, Clone, Default, PartialEq, RuntimeDebug)]
    pub struct SCustomer<Hash> {
        pub id: u32,
        pub name: Hash,
        pub location: (u32, u32),
    }
    // DriverOf is datatype used for storage in Driver
    type DriverOf<T> = SDriver<<T as frame_system::Config>::Hash>;
    //SDriver is a struct for Driver
    #[derive(Encode, Decode, Copy, Clone, Default, PartialEq, RuntimeDebug)]
    pub struct SDriver<Hash> {
        pub id: u32,
        pub car_no: Hash,
        pub location: (u32, u32),
        pub price: u32,
    }
    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // The pallet's runtime storage items.
    // https://substrate.dev/docs/en/knowledgebase/runtime/storage
    #[pallet::storage]
    #[pallet::getter(fn get_customer)]
    pub type Customer<T: Config> = StorageMap<_, Blake2_128Concat, u32, CustomerOf<T>>;
    #[pallet::storage]
    #[pallet::getter(fn get_driver)]
    pub type Driver<T: Config> = StorageMap<_, Blake2_128Concat, u32, DriverOf<T>>;
    #[pallet::storage]
    #[pallet::getter(fn get_booking)]
    pub type Booking<T: Config> = StorageMap<_, Blake2_128Concat, u32, u32>;
    // Learn more about declaring storage items:
    // https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items

    // Pallets use events to inform users when important changes are made.
    // https://substrate.dev/docs/en/knowledgebase/runtime/events
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // event emitted when driver's location is updated.
        DriverLocationUpdated(T::AccountId, u32),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        // Error emitted when driver is not found in storage.
        DriverDoesNotExist,
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// An example dispatchable that takes a singles value as a parameter, writes the value to
        /// storage and emits an event. This function must be dispatched by a signed extrinsic.
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn update_cab_location(
            origin: OriginFor<T>,
            driver_id: u32,
            location: (u32, u32),
        ) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin

            let who = ensure_signed(origin)?;
            ensure!(
                <Driver<T>>::contains_key(&driver_id),
                Error::<T>::DriverDoesNotExist
            );
            let new_driver_option = <Driver<T>>::get(&driver_id);
            let mut new_driver = new_driver_option.unwrap();
            new_driver.location.0 = location.0;
            new_driver.location.1 = location.1;
            <Driver<T>>::insert(&driver_id, new_driver);

            Self::deposit_event(Event::DriverLocationUpdated(who, new_driver.id));
            Ok(().into())
        }
    }
}
