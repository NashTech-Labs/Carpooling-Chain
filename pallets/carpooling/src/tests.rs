use crate::{Error, mock::*};
use crate::SCustomer;
use sp_core::Encode;
use sp_runtime::traits::Hash;
use frame_support::{assert_ok, assert_noop};

#[test]
fn add_new_customer() {
	new_test_ext().execute_with(|| {
		let new_cust = SCustomer {
			id:32,
			name: "Ankit".using_encoded(<Test as frame_system::Config>::Hashing::hash),
			location: "New Delhi".using_encoded(<Test as frame_system::Config>::Hashing::hash),
		};
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::add_new_customer(Origin::signed(1), 42, new_cust));
		// Read pallet storage and assert an expected result.
		//assert_eq!(TemplateModule::get_customer(42), Some(&new_cust));
	});
}
