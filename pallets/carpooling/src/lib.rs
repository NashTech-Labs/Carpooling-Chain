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
    #[derive(Encode, Decode, Clone, Default, PartialEq, RuntimeDebug)]
    pub struct SCustomer<Hash> {
        pub id: u32,
        pub name: Hash,
        pub location: Hash,
    }
    // DriverOf is datatype used for storage in Driver
    type DriverOf<T> = SDriver<<T as frame_system::Config>::Hash>;
    //SDriver is a struct for Driver
    #[derive(Encode, Decode, Clone, Default, PartialEq, RuntimeDebug)]
    pub struct SDriver<Hash> {
        pub id: u32,
        pub car_no: Hash,
        pub location: Hash,
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
    pub type Booking<T: Config> = StorageMap<_, Blake2_128Concat, u32, T::AccountId>;
    // Learn more about declaring storage items:
    // https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items

    // Pallets use events to inform users when important changes are made.
    // https://substrate.dev/docs/en/knowledgebase/runtime/events
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        CustomerAdded(u32, T::AccountId),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Error names should be descriptive.
        CustomerAlreadyExist,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// An example dispatchable that takes a singles value as a parameter, writes the value to
        /// storage and emits an event. This function must be dispatched by a signed extrinsic.
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn add_new_customer(
            origin: OriginFor<T>,
            cust_id: u32,
            new_cust: CustomerOf<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            match <Customer<T>>::get(cust_id) {
                Some(_) => Err(Error::<T>::CustomerAlreadyExist)?,
                None => {
                    <Customer<T>>::insert(cust_id, new_cust);
                }
            }

            // Emit an event.
            Self::deposit_event(Event::CustomerAdded(cust_id, who));
            // Return a successful DispatchResultWithPostInfo
            Ok(())
        }
    }
}
