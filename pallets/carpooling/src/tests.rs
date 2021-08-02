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
#[test]
fn add_new_cab() {
    new_test_ext().execute_with(|| {
        let new_cab = SDriver {
            id: 32,
            car_no: "2345".using_encoded(<Test as frame_system::Config>::Hashing::hash),
            location: (20, 30),
            price: 12,
        };
        // Dispatch a signed extrinsic.
        assert_eq!(
            Carpooling::add_new_cab(Origin::signed(1), 42, new_cab),
            Ok(())
        );
        // Read pallet storage and assert an expected result.
    });
}
#[test]
fn add_new_cab_fails() {
    new_test_ext().execute_with(|| {
        let new_cab = SDriver {
            id: 32,
            car_no: "2345".using_encoded(<Test as frame_system::Config>::Hashing::hash),
            location: (20, 30),
            price: 12,
        };
        // Dispatch a signed extrinsic.
        assert_ok!(Carpooling::add_new_cab(Origin::signed(1), 42, new_cab));
        let new_cab_1 = SDriver {
            id: 32,
            car_no: "2345".using_encoded(<Test as frame_system::Config>::Hashing::hash),
            location: (20, 30),
            price: 12,
        };
        assert_eq!(
            Carpooling::add_new_cab(Origin::signed(1), 42, new_cab_1),
            Err(DispatchError::Module {
                index: 1,
                error: 1,
                message: Some("CabAlreadyExist")
            })
        );
    });
}

#[test]
fn update_customer_location_success() {
    new_test_ext().execute_with(|| {
        let old_cust = SCustomer {
            id: 20,
            name: "ABC".using_encoded(<Test as frame_system::Config>::Hashing::hash),
            location: (20, 30),
        };
        let new_location: (u32, u32) = (70, 60);
        let new_cust = SCustomer {
            id: 20,
            name: "ABC".using_encoded(<Test as frame_system::Config>::Hashing::hash),
            location: new_location,
        };
        Customer::<Test>::insert(10, old_cust);
        assert_ok!(Carpooling::update_customer_location(
            Origin::signed(10),
            10,
            new_location
        ));
        assert_eq!(<Customer<Test>>::get(10), Some(new_cust));
    });
}
#[test]
fn update_customer_location_fail() {
    new_test_ext().execute_with(|| {
        let new_location: (u32, u32) = (40, 50);
        assert_eq!(
            Carpooling::update_customer_location(Origin::signed(1), 42, new_location),
            Err(DispatchError::Module {
                index: 1,
                error: 3,
                message: Some("CustomerDoesNotExist",),
            })
        )
    });
}