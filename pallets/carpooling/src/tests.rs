use super::*;
use crate::mock::*;
use frame_support::assert_ok;
use frame_support::pallet_prelude::Encode;
use sp_runtime::traits::Hash;
use sp_runtime::DispatchError;

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
        assert_eq!(Carpooling::add_new_cab(Origin::signed(1), 42, new_cab_1) ,
            Err(DispatchError::Module {index: 1,error: 0,message: Some("CabAlreadyExist")})
        );
    });
}
/*

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			TemplateModule::cause_error(Origin::signed(1)),
			Error::<Test>::NoneValue
		);
	});
}
*/