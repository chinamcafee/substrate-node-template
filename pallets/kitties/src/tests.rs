use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;

#[test]
fn owned_kitties_can_append_values() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        assert_eq!(KittiesModule::create(Origin::signed(1)), Ok(()));
    })
}

#[test]
fn transfer_kitties() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        KittiesModule::create(Origin::signed(1));
        let receiver = Origin::signed(2);
        let receiverId = ensure_signed(receiver).unwrap();
        assert_eq!(KittiesModule::transfer(Origin::signed(1), receiverId, 1), Ok(()));
    })
}


#[test]
fn breed_kitties() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        KittiesModule::create(Origin::signed(1));
        KittiesModule::create(Origin::signed(1));
        assert_noop!(KittiesModule::breed(Origin::signed(1), 1, 2), Error::<Test>::InvalidKittyId);
    })
}


#[test]
fn breed_kitties_same_parent() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        KittiesModule::create(Origin::signed(1));
        KittiesModule::create(Origin::signed(1));
        assert_noop!(KittiesModule::breed(Origin::signed(1), 1, 1), Error::<Test>::RequireDifferentParent);
    })
}

