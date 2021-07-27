use super::*;
use crate::mock::*;
use frame_support::assert_ok;
use frame_support::pallet_prelude::Encode;
use sp_runtime::traits::Hash;
use sp_runtime::DispatchError;

#[test]
fn check_storage() {
    new_test_ext().execute_with(|| {
        let cust = SCustomer {
            id: 20,
            name: "aman".using_encoded(<Test as frame_system::Config>::Hashing::hash),
            location: (20, 30),
        };
        Customer::<Test>::insert(10, &cust);
        assert_eq!(<Customer<Test>>::get(10), Some(cust));
    });
}

#[test]
fn update_cab_location_success() {
    new_test_ext().execute_with(|| {
        let old_driver = SDriver {
            id: 20,
            car_no: "UP76 E 8559".using_encoded(<Test as frame_system::Config>::Hashing::hash),
            location: (20, 30),
            price: 20,
        };
        let new_location: (u32, u32) = (40, 40);
        let new_driver = SDriver {
            id: 20,
            car_no: "UP76 E 8559".using_encoded(<Test as frame_system::Config>::Hashing::hash),
            location: new_location,
            price: 20,
        };
        Driver::<Test>::insert(10, old_driver);
        assert_ok!(Carpooling::update_cab_location(
            Origin::signed(10),
            10,
            new_location
        ));
        assert_eq!(<Driver<Test>>::get(10), Some(new_driver));
    });
}
#[test]
fn update_cab_location_fail() {
    new_test_ext().execute_with(|| {
        let new_location: (u32, u32) = (40, 50);
        assert_eq!(
            Carpooling::update_cab_location(Origin::signed(1), 42, new_location),
            Err(DispatchError::Module {
                index: 1,
                error: 0,
                message: Some("DriverDoesNotExist",),
            })
        )
    });
}
