use core::convert::TryInto;

use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};



#[test]
fn create_profile_works() {
	new_test_ext().execute_with(|| {
		// Create vector of interests
		let mut vec = Vec::new();
		vec.push(7);
		const USERNAME:&'static [u8] = &[1];
		let available_hours_per_week = 40_u8;

		// Ensure the user can create profile
		assert_ok!(Profile::create_profile(Origin::signed(1), USERNAME.to_vec().try_into().unwrap(), vec.try_into().unwrap(), available_hours_per_week));
	});
}

#[test]
fn verify_inputs_outputs_to_profile(){
	new_test_ext().execute_with( || {
		// Assign values to profile properties
		const USERNAME:&'static [u8] = &[1];
		const INTERESTS:&'static [u8] = &[7];
		let available_hours_per_week = 40_u8;

		// Create Profile
		assert_ok!(Profile::create_profile(Origin::signed(10), USERNAME.to_vec().try_into().unwrap(), INTERESTS.to_vec().try_into().unwrap(), available_hours_per_week));

		// Get profile for current account
		let profile = Profile::profiles(10).expect("should found the profile");

		// Ensure that profile properties are assigned correctly
		assert_eq!(profile.name.into_inner(), &[1]);
		assert_eq!(profile.reputation, 0);
		assert_eq!(profile.interests.into_inner(), &[7]);
	});
}

#[test]
fn create_profile_increases_profile_count() {
	new_test_ext().execute_with(|| {
		// Create vector of interests
		let mut vec = Vec::new();
		vec.push(7);
		const USERNAME:&'static [u8] = &[1];
		let available_hours_per_week = 40_u8;

		// Ensure the user can create profile
		assert_ok!(Profile::create_profile(Origin::signed(1), USERNAME.to_vec().try_into().unwrap(), vec.try_into().unwrap(), available_hours_per_week));

		// Ensure count has decreased
		assert_eq!(Profile::profile_count(), 1);
	});
}

#[test]
fn only_one_profile_per_account_allowed() {
	new_test_ext().execute_with(|| {
		// Create vector of interests
		let mut vec = Vec::new();
		vec.push(7);
		const USERNAME:&'static [u8] = &[1];
		let available_hours_per_week = 40_u8;

		// Ensure the user can create profile
		assert_ok!(Profile::create_profile(Origin::signed(1), USERNAME.to_vec().try_into().unwrap(), vec.try_into().unwrap(), available_hours_per_week));

		// Create vector of interests
		let mut vec = Vec::new();
		vec.push(7);

		assert_noop!(Profile::create_profile(Origin::signed(1), USERNAME.to_vec().try_into().unwrap(), vec.try_into().unwrap(), available_hours_per_week), Error::<Test>::ProfileAlreadyCreated );
	});
}

#[test]
fn delete_profile_works() {
	new_test_ext().execute_with(|| {
		// Create vector of interests
		let mut vec = Vec::new();
		vec.push(7);
		const USERNAME:&'static [u8] = &[1];
		let available_hours_per_week = 40_u8;

		// Ensure the user can create profile
		assert_ok!(Profile::create_profile(Origin::signed(1), USERNAME.to_vec().try_into().unwrap(), vec.try_into().unwrap(), available_hours_per_week));

		// Ensure the user can delete their profile
		assert_ok!(Profile::remove_profile(Origin::signed(1)));
	});
}

#[test]
fn delete_profile_decreases_profile_count() {
	new_test_ext().execute_with(|| {
		// Create vector of interests
		let mut vec = Vec::new();
		vec.push(7);
		const USERNAME:&'static [u8] = &[1];
		let available_hours_per_week = 40_u8;

		// Ensure the user can create profile
		assert_ok!(Profile::create_profile(Origin::signed(1), USERNAME.to_vec().try_into().unwrap(), vec.try_into().unwrap(), available_hours_per_week));

		// Ensure teh user can delete their profile
		assert_ok!(Profile::remove_profile(Origin::signed(1)));

		// Ensure count is reduced when removing profile
		assert_eq!(Profile::profile_count(), 0);
	});
}

#[test]
fn user_can_only_delete_own_profile() {
	new_test_ext().execute_with(|| {
		// Create vector of interests
		let mut vec = Vec::new();
		vec.push(7);
		const USERNAME:&'static [u8] = &[1];
		let available_hours_per_week = 40_u8;

		// Ensure the user can create profile
		assert_ok!(Profile::create_profile(Origin::signed(1), USERNAME.to_vec().try_into().unwrap(),  vec.try_into().unwrap(), available_hours_per_week));

		// Ensure another user can NOT delete others profile
		assert_noop!(Profile::remove_profile(Origin::signed(2)), Error::<Test>::NoProfileCreated);

		// Ensure count is NOT reduced when removing profile
		assert_eq!(Profile::profile_count(), 1);
	});
}

#[test]
fn user_can_update_profile() {
	new_test_ext().execute_with(|| {
		// Create profile properties
		let interests = vec![1];
		let username = vec![1];
		let available_hours_per_week = 40_u8;

		// Ensure the user can create profile
		assert_ok!(Profile::create_profile(Origin::signed(10), username.to_vec().try_into().unwrap(), interests.to_vec().try_into().unwrap(), available_hours_per_week));

		// Create new vector of interests
		let interests = vec![6];
		let username =  vec![7];
		let available_hours_per_week = 20_u8;
		let additional_information = vec![0; 5000];
		// Ensure user can update profile with new interests
		assert_ok!(Profile::update_profile(Origin::signed(10), username.to_vec().try_into().unwrap(), interests.to_vec().try_into().unwrap(), available_hours_per_week, Some(additional_information.try_into().unwrap())));

		// Get profile for current account
		let profile = Profile::profiles(10).expect("should found the profile");

		// Ensure count is NOT reduced when removing profile
		assert_eq!(Profile::profile_count(), 1);

		// Ensure that the values have been updated successfully
		assert_eq!(profile.name.into_inner(), &[7]);
		assert_eq!(profile.interests.into_inner(), &[6]);
		assert_eq!(profile.available_hours_per_week, 20_u8);

	});
}

#[test]
fn user_can_only_update_own_profile() {
	new_test_ext().execute_with(|| {
		// Create vector of interests
		let mut vec = Vec::new();
		vec.push(7);
		const USERNAME:&'static [u8] = &[1];
		let available_hours_per_week = 40_u8;

		// Ensure the user can create profile
		assert_ok!(Profile::create_profile(Origin::signed(1), USERNAME.to_vec().try_into().unwrap(), vec.try_into().unwrap(), available_hours_per_week));

		// Create new vector of interests
		let mut vec2 = Vec::new();
		vec2.push(99);
		let available_hours_per_week = 20_u8;
		let additional_information = vec![0; 5000];

		// Ensure another user can NOT update others profile.
		assert_noop!(Profile::update_profile(Origin::signed(2), USERNAME.to_vec().try_into().unwrap(), vec2.try_into().unwrap(), available_hours_per_week, Some(additional_information.try_into().unwrap())), Error::<Test>::NoProfileCreated);
	});
}
