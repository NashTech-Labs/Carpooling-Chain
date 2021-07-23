use crate::{mock::*, SCustomer};
use frame_support::assert_ok;
use frame_support::sp_runtime::DispatchError::Module;
use sp_core::Encode;
use sp_runtime::traits::Hash;

#[test]
fn add_new_customer() {
    new_test_ext().execute_with(|| {
        let new_cust = SCustomer {
            id: 32,
            name: "Ankit".using_encoded(<Test as frame_system::Config>::Hashing::hash),
            location: "New Delhi".using_encoded(<Test as frame_system::Config>::Hashing::hash),
        };
        // Dispatch a signed extrinsic.
        assert_eq!(
            TemplateModule::add_new_customer(Origin::signed(1), 42, new_cust),
            Ok(())
        );
        // Read pallet storage and assert an expected result.
    });
}

#[test]
fn add_new_customer_fails() {
    new_test_ext().execute_with(|| {
        let new_cust = SCustomer {
            id: 32,
            name: "Ankit".using_encoded(<Test as frame_system::Config>::Hashing::hash),
            location: "New Delhi".using_encoded(<Test as frame_system::Config>::Hashing::hash),
        };
        // Dispatch a signed extrinsic.
        assert_ok!(TemplateModule::add_new_customer(
            Origin::signed(1),
            42,
            new_cust
        ));
        let new_cust_1 = SCustomer {
            id: 32,
            name: "Ankit".using_encoded(<Test as frame_system::Config>::Hashing::hash),
            location: "New Delhi".using_encoded(<Test as frame_system::Config>::Hashing::hash),
        };
        assert_eq!(
            TemplateModule::add_new_customer(Origin::signed(1), 42, new_cust_1),
            Err(Module {
                index: 1,
                error: 0,
                message: Some("CustomerAlreadyExist")
            })
        );
    });
}

#[test]
fn check_storage() {
    new_test_ext().execute_with(|| {
        let new_cust = SCustomer {
            id: 32,
            name: "Ankit".using_encoded(<Test as frame_system::Config>::Hashing::hash),
            location: "New Delhi".using_encoded(<Test as frame_system::Config>::Hashing::hash),
        };
        let new_cust_1 = new_cust.clone();
        assert_ok!(TemplateModule::add_new_customer(
            Origin::signed(1),
            42,
            new_cust
        ));
        assert_eq!(TemplateModule::get_customer(42), Some(new_cust_1));
    })
}
