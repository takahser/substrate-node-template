use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use sp_core::sr25519;



#[test]
fn can_create_vision() {
	new_test_ext().execute_with(|| {
		// Create Vision Document
		const VISION: &'static [u8] = &[7];

		// Ensure the DAO can create a vision document
		assert_ok!(Dao::create_vision(Origin::signed(*ALICE), VISION.to_vec()));
	});
}

#[test]
fn creating_vision_increases_vision_count() {
	new_test_ext().execute_with(|| {
		// Create Vision Document
		const VISION: &'static [u8] = &[7];

		// Ensure the DAO can create a vision document
		assert_ok!(Dao::create_vision(Origin::signed(*ALICE), VISION.to_vec()));

		// Ensure vision count is 1
		assert_eq!(Dao::vision_count(), 1);
	});
}

#[test]
fn can_not_create_vision_that_already_exists() {
	new_test_ext().execute_with(|| {

		// Create Vision Document
		const VISION: &'static [u8] = &[7];

		// Ensure the DAO can create a vision document
		assert_ok!(Dao::create_vision(Origin::signed(*ALICE), VISION.to_vec()));

		// Ensure the DAO can NOT Create create a vision that already exists
		assert_noop!(Dao::create_vision(Origin::signed(*ALICE), VISION.to_vec()), Error::<Test>::VisionAlreadyExists);
	});
}

#[test]
fn can_remove_vision() {
	new_test_ext().execute_with(|| {

		// Create Vision Document
		const VISION: &'static [u8] = &[7];

		// Ensure the DAO can create a vision document
		assert_ok!(Dao::create_vision(Origin::signed(*ALICE), VISION.to_vec()));

		// Ensure the DAO can remove a vision document
		assert_ok!(Dao::remove_vision(Origin::signed(*ALICE), VISION.to_vec()));

		// TODO: Enforce stronger check on Vision test
		assert_eq!(Dao::vision(VISION.to_vec()).0, sr25519::Public::from_raw([0_u8; 32]));
	});
}

#[test]
fn removing_vision_decreases_vision_count() {
	new_test_ext().execute_with(|| {
		// Create Vision Document
		const VISION: &'static [u8] = &[7];

		// Ensure the DAO can create a vision document
		assert_ok!(Dao::create_vision(Origin::signed(*ALICE), VISION.to_vec()));

		// Ensure vision count is 1
		assert_eq!(Dao::vision_count(), 1);

		// Ensure the DAO can remove a vision document
		assert_ok!(Dao::remove_vision(Origin::signed(*ALICE), VISION.to_vec()));

		// Ensure vision count is 0
		assert_eq!(Dao::vision_count(), 0);
	});
}

#[test]
fn when_removing_vision_ensure_it_exists() {
	new_test_ext().execute_with(|| {

		// Create Vision Document
		const VISION: &'static [u8] = &[7];

		// Ensure error is thrown when no vision exists yet
		assert_noop!(Dao::remove_vision(Origin::signed(*ALICE), VISION.to_vec()), Error::<Test>::NoSuchVision);
	});
}

#[test]
fn only_vision_owner_can_remove_vision() {
	new_test_ext().execute_with(|| {

		// Create Vision Document
		const VISION: &'static [u8] = &[7];

		// Ensure the DAO can create a vision document
		assert_ok!(Dao::create_vision(Origin::signed(*ALICE), VISION.to_vec()));

		// Ensure the vision can not be deleted by user who didn't create it. Created with user 1, deleted with 2
		assert_noop!(Dao::remove_vision(Origin::signed(*BOB), VISION.to_vec()), Error::<Test>::NotVisionOwner);
	});
}

#[test]
fn user_can_sign_onto_vision() {
	new_test_ext().execute_with(|| {

		// Create Static Vision
		const VISION: &'static [u8] = &[1];

		// Ensure the DAO can create a vision document
		assert_ok!(Dao::create_vision(Origin::signed(*ALICE), VISION.to_vec()));

		// Ensure a user can sign onto vision.
		assert_ok!(Dao::sign_vision(Origin::signed(*ALICE), VISION.to_vec()));

		// Ensure the length of VisionSigner has increased
		assert_eq!(Dao::applicants_to_organization(VISION.to_vec()).len(), 1);
	});
}

#[test]
fn user_can_unsign_from_vision() {
	new_test_ext().execute_with(|| {

		// Create Static Vision
		const VISION: &'static [u8] = &[1];

		// Ensure the DAO can create a vision document
		assert_ok!(Dao::create_vision(Origin::signed(*ALICE), VISION.to_vec()));

		// Ensure a user can sign onto vision.
		assert_ok!(Dao::sign_vision(Origin::signed(*BOB), VISION.to_vec()));

		// Ensure the length of VisionSigners has increased
		assert_eq!(Dao::applicants_to_organization(VISION.to_vec()).len(), 1);

		// Ensure a user can unsign onto vision.
		assert_ok!(Dao::unsign_vision(Origin::signed(*BOB), VISION.to_vec()));

		// Ensure the length of VisionSigners has increased
		assert_eq!(Dao::applicants_to_organization(VISION.to_vec()).len(), 0);
	});
}

#[test]
fn user_can_sign_onto_vision_if_vision_exists() {
	new_test_ext().execute_with(|| {

		// Create Vision
		const VISION: &'static [u8] = &[1];

		// Ensure the DAO can create a vision document
		assert_ok!(Dao::create_vision(Origin::signed(*ALICE), VISION.to_vec()));

		// Ensure Error is thrown if vision doesn't exist when signing
		assert_noop!(Dao::sign_vision(Origin::signed(*ALICE), Vec::new()), Error::<Test>::NoSuchVision );

	});
}

#[test]
fn user_can_unsign_from_vision_if_vision_exists() {
	new_test_ext().execute_with(|| {

		// Create Vision
		const VISION: &'static [u8] = &[1];

		// Ensure the DAO can create a vision document
		assert_ok!(Dao::create_vision(Origin::signed(*ALICE), VISION.to_vec()));

		// Ensure Error is thrown if vision doesn't exist when unsigning
		assert_noop!(Dao::unsign_vision(Origin::signed(*ALICE), Vec::new()), Error::<Test>::NoSuchVision );

	});
}

#[test]
fn user_can_sign_onto_vision_only_if_not_signed_previously() {
	new_test_ext().execute_with(|| {

		// Create Vision Document
		const VISION: &'static [u8] = &[1];

		// Ensure the DAO can create a vision document
		assert_ok!(Dao::create_vision(Origin::signed(*ALICE), VISION.to_vec()));

		// Ensure Vision can be signed
		assert_ok!(Dao::sign_vision(Origin::signed(*BOB), VISION.to_vec()));

		// Ensure Error is thrown if vision is already signed
		assert_noop!(Dao::sign_vision(Origin::signed(*BOB), VISION.to_vec()), Error::<Test>::AlreadySigned );

	});
}

#[test]
fn user_can_unsign_from_vision_only_if_signed_previously() {
	new_test_ext().execute_with(|| {

		// Create Vision Document
		const VISION: &'static [u8] = &[1];

		// Ensure the DAO can create a vision document
		assert_ok!(Dao::create_vision(Origin::signed(*ALICE), VISION.to_vec()));

		// Ensure Error is thrown if vision has not been signed previously
		assert_noop!(Dao::unsign_vision(Origin::signed(*BOB), VISION.to_vec()), Error::<Test>::NotSigned );

	});
}

#[test]
fn can_create_an_organization() {
	new_test_ext().execute_with(|| {

		// Create Static Organization name, description, vision
		const ORG_NAME: &'static [u8] = &[10];
		const ORG_DESC: &'static [u8] = &[10];
		const ORG_VISION: &'static [u8] = &[10];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(*ALICE), ORG_NAME.to_vec(),
		ORG_DESC.to_vec(), ORG_VISION.to_vec()));

		let org_id = Dao::organization_at_index(1).expect("must exist one org");

		let members = Dao::members(org_id);

		// Ensure the length of organization is equal to 1, and that member is creater of org.
		assert_eq!(members, &[*ALICE]);
	});
}

#[test]
fn creating_organization_increases_organization_count() {
	new_test_ext().execute_with(|| {
		// Create Static Organization name, description, vision
		const ORG_NAME: &'static [u8] = &[10];
		const ORG_DESC: &'static [u8] = &[10];
		const ORG_VISION: &'static [u8] = &[10];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(*ALICE), ORG_NAME.to_vec(),
		ORG_DESC.to_vec(), ORG_VISION.to_vec()));

		// Ensure organization count is 1
		assert_eq!(Dao::organization_count(), 1);
	});
}

#[test]
fn can_create_multiple_organization() {
	new_test_ext().execute_with(|| {

		// Create Static Organization name, description, vision
		const ORG_NAME1: &'static [u8] = &[10];
		const ORG_DESC1: &'static [u8] = &[10];
		const ORG_VISION1: &'static [u8] = &[10];
		const ORG_NAME2: &'static [u8] = &[1];
		const ORG_DESC2: &'static [u8] = &[1];
		const ORG_VISION2: &'static [u8] = &[1];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(*ALICE), ORG_NAME1.to_vec(),
		ORG_DESC1.to_vec(), ORG_VISION1.to_vec()));

		// Ensure second organization can be created by a different user
		assert_ok!(Dao::create_organization(Origin::signed(*BOB), ORG_NAME2.to_vec(),
		ORG_DESC2.to_vec(), ORG_VISION2.to_vec()));

		let org_id_1 = Dao::organization_at_index(1).expect("must exist one org");
		let org_id_2 = Dao::organization_at_index(2).expect("must exist one org");
		// Ensure each organization was created successfully
		assert_eq!(Dao::members(org_id_1).len(), 1);
		assert_eq!(Dao::members(org_id_2).len(), 1);

		// Ensure organization count is 2
		assert_eq!(Dao::organization_count(), 2);
	});
}


#[test]
fn can_remove_an_organization() {
	new_test_ext().execute_with(|| {

		// Create Static Organization name, description, vision
		const ORG_NAME: &'static [u8] = &[10];
		const ORG_DESC: &'static [u8] = &[10];
		const ORG_VISION: &'static [u8] = &[10];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(*ALICE), ORG_NAME.to_vec(),
		ORG_DESC.to_vec(), ORG_VISION.to_vec()));

		let org_id = Dao::organization_at_index(1).expect("must exist one org");
		// Ensure the length of organization is equal to 1
		assert_eq!(Dao::members(org_id).len(), 1);

		// Ensure organization can be removed
		assert_ok!(Dao::dissolve_organization(Origin::signed(*ALICE), org_id));

		// Ensure the organization has been removed by checking the length
		assert_eq!(Dao::members(org_id).len(), 0);
	});
}

#[test]
fn removing_organization_decreases_organization_count() {
	new_test_ext().execute_with(|| {

		// Create Static Organization name, description, vision
		const ORG_NAME: &'static [u8] = &[10];
		const ORG_DESC: &'static [u8] = &[10];
		const ORG_VISION: &'static [u8] = &[10];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(*ALICE), ORG_NAME.to_vec(),
		ORG_DESC.to_vec(), ORG_VISION.to_vec()));

		let org_id = Dao::organization_at_index(1).expect("must exist one org");
		// Ensure the length of organization is equal to 1
		assert_eq!(Dao::members(org_id).len(), 1);

		assert_eq!(Dao::organization_count(), 1);

		// Ensure organization can be removed
		assert_ok!(Dao::dissolve_organization(Origin::signed(*ALICE), org_id));
		// Ensure the organization has been removed by checking the length
		assert_eq!(Dao::members(org_id).len(), 0);

		// Ensure organization count is 0
		assert_eq!(Dao::organization_count(), 0);
	});
}

#[test]
fn only_creator_can_remove_their_organization() {
	new_test_ext().execute_with(|| {

		// Create Static Organization name, description, vision
		const ORG_NAME: &'static [u8] = &[10];
		const ORG_DESC: &'static [u8] = &[10];
		const ORG_VISION: &'static [u8] = &[10];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(*ALICE), ORG_NAME.to_vec(),
		ORG_DESC.to_vec(), ORG_VISION.to_vec()));

		let org_id = Dao::organization_at_index(1).expect("must exist one org");
		// Ensure the length of organization is equal to 1
		assert_eq!(Dao::members(org_id).len(), 1);

		// Ensure organization can't be removed by another member. Only creator can remove their own org
		assert_noop!(Dao::dissolve_organization(Origin::signed(*BOB), org_id), Error::<Test>::NotOrganizationCreator);

		// Ensure the organization has not been deleted
		assert_eq!(Dao::members(org_id).len(), 1);

	});
}

#[test]
fn can_add_user_to_organization() {
	new_test_ext().execute_with(|| {

		// Create Static Organization name, description, vision
		const ORG_NAME: &'static [u8] = &[10];
		const ORG_DESC: &'static [u8] = &[10];
		const ORG_VISION: &'static [u8] = &[10];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(*ALICE), ORG_NAME.to_vec(),
		ORG_DESC.to_vec(), ORG_VISION.to_vec()));

		let org_id = Dao::organization_at_index(1).expect("must exist one org");
		// Ensure the length of organization is equal to 1
		assert_eq!(Dao::members(org_id).len(), 1);
		// Ensure users can be added to a DAO
		assert_ok!(Dao::add_members(Origin::signed(*ALICE), org_id, *EVE));

		// Ensure the organization has 2 members (creator abd user4)
		assert_eq!(Dao::members(org_id).len(), 2);

	});
}

#[test]
fn only_creator_can_add_user_to_organization() {
	new_test_ext().execute_with(|| {

		// Create Static Organization name, description, vision
		const ORG_NAME: &'static [u8] = &[10];
		const ORG_DESC: &'static [u8] = &[10];
		const ORG_VISION: &'static [u8] = &[10];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(*ALICE), ORG_NAME.to_vec(),
		ORG_DESC.to_vec(), ORG_VISION.to_vec()));

		let org_id = Dao::organization_at_index(1).expect("must exist one org");
		// Ensure the length of organization is equal to 1
		assert_eq!(Dao::members(org_id).len(), 1);
		// Throw error if another than Creator is trying to add members
		assert_noop!(Dao::add_members(Origin::signed(*BOB), org_id, *EVE), Error::<Test>::NotOrganizationCreator);
	});
}


#[test]
fn can_only_add_members_if_not_already_in_organization() {
	new_test_ext().execute_with(|| {

		// Create Static Organization name, description, vision
		const ORG_NAME: &'static [u8] = &[10];
		const ORG_DESC: &'static [u8] = &[10];
		const ORG_VISION: &'static [u8] = &[10];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(*ALICE), ORG_NAME.to_vec(),
		ORG_DESC.to_vec(), ORG_VISION.to_vec()));

		let org_id = Dao::organization_at_index(1).expect("must exist one org");
		// Ensure the length of organization is equal to 1
		assert_eq!(Dao::members(org_id).len(), 1);
		// Throw error if another than Creator is trying to add members
		assert_ok!(Dao::add_members(Origin::signed(*ALICE), org_id, *BOB));

		// Ensure adding existing member throws an error
		assert_noop!(Dao::add_members(Origin::signed(*ALICE), org_id, *BOB), Error::<Test>::AlreadyMember);
	});
}

#[test]
fn organization_exists_check_before_adding_user_to_org() {
	new_test_ext().execute_with(|| {

		// Create Static Organization name, description, vision
		const ORG_NAME: &'static [u8] = &[10];
		const ORG_DESC: &'static [u8] = &[10];
		const ORG_VISION: &'static [u8] = &[10];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(*ALICE), ORG_NAME.to_vec(),
		ORG_DESC.to_vec(), ORG_VISION.to_vec()));

		let org_id = Dao::organization_at_index(1).expect("must exist one org");
		// Ensure the length of organization is equal to 1
		assert_eq!(Dao::members(org_id).len(), 1);
		// dissolve_organization
		assert_ok!(Dao::dissolve_organization(Origin::signed(*ALICE), org_id));
		// Throw error if org_id is not found
		assert_noop!(Dao::add_members(Origin::signed(*ALICE), org_id, *EVE), Error::<Test>::InvalidOrganization);
	});
}

#[test]
fn only_creator_can_remove_users_from_organization() {
	new_test_ext().execute_with(|| {

		// Create Static Organization name, description, vision
		const ORG_NAME: &'static [u8] = &[10];
		const ORG_DESC: &'static [u8] = &[10];
		const ORG_VISION: &'static [u8] = &[10];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(*ALICE), ORG_NAME.to_vec(),
		ORG_DESC.to_vec(), ORG_VISION.to_vec()));

		let org_id = Dao::organization_at_index(1).expect("must exist one org");
		// Ensure the length of organization is equal to 1
		assert_eq!(Dao::members(org_id).len(), 1);

		// Ensure users can be added to a DAO
		assert_ok!(Dao::add_members(Origin::signed(*ALICE), org_id, *EVE));

		// When user 2 who didn't create organization tries to remove user, throw error
		assert_noop!(Dao::remove_members(Origin::signed(*BOB), org_id, *EVE), Error::<Test>::NotOrganizationCreator);

	});
}

#[test]
fn organization_exists_check_before_removing_user_from_org() {
	new_test_ext().execute_with(|| {

		// Create Static Organization name, description, vision
		const ORG_NAME: &'static [u8] = &[10];
		const ORG_DESC: &'static [u8] = &[10];
		const ORG_VISION: &'static [u8] = &[10];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(*ALICE), ORG_NAME.to_vec(),
		ORG_DESC.to_vec(), ORG_VISION.to_vec()));

		let org_id = Dao::organization_at_index(1).expect("must exist one org");
		// Ensure the length of organization is equal to 1
		assert_eq!(Dao::members(org_id).len(), 1);

		// Ensure users can be added to a DAO
		assert_ok!(Dao::add_members(Origin::signed(*ALICE), org_id, *EVE));
		// dissolve_organization
		assert_ok!(Dao::dissolve_organization(Origin::signed(*ALICE), org_id));

		// Ensure error is thrown when removing members from non-existing organization
		assert_noop!(Dao::remove_members(Origin::signed(*ALICE), org_id, *EVE), Error::<Test>::InvalidOrganization);
	});
}

#[test]
fn can_remove_users_from_organization() {
	new_test_ext().execute_with(|| {

		// Create Static Organization name, description, vision
		const ORG_NAME: &'static [u8] = &[10];
		const ORG_DESC: &'static [u8] = &[10];
		const ORG_VISION: &'static [u8] = &[10];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(*ALICE), ORG_NAME.to_vec(),
		ORG_DESC.to_vec(), ORG_VISION.to_vec()));

		let org_id = Dao::organization_at_index(1).expect("must exist one org");
		// Ensure the length of organization is equal to 1
		assert_eq!(Dao::members(org_id).len(), 1);

		// Ensure users can be added to a DAO
		assert_ok!(Dao::add_members(Origin::signed(*ALICE), org_id, *EVE));
		assert_ok!(Dao::add_members(Origin::signed(*ALICE), org_id, *JOHN));

		// User can be removed from organization
		assert_ok!(Dao::remove_members(Origin::signed(*ALICE), org_id, *EVE));

		// Validate Ensure length of users in org is 2
		assert_eq!(Dao::members(org_id).len(), 2);

	});
}

#[test]
fn can_only_remove_users_that_belong_to_organization() {
	new_test_ext().execute_with(|| {


		// Create Static Organization name, description, vision
		const ORG_NAME: &'static [u8] = &[10];
		const ORG_DESC: &'static [u8] = &[10];
		const ORG_VISION: &'static [u8] = &[10];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(*ALICE), ORG_NAME.to_vec(),
		ORG_DESC.to_vec(), ORG_VISION.to_vec()));

		let org_id = Dao::organization_at_index(1).expect("must exist one org");
		// Ensure the length of organization is equal to 1
		assert_eq!(Dao::members(org_id).len(), 1);
		// Ensure users can be added to a DAO
		assert_ok!(Dao::add_members(Origin::signed(*ALICE), org_id, *EVE));

		// Ensure length of users in org is 2
		assert_eq!(Dao::members(org_id).len(), 2);

		// Ensure error is thrown if user is not in organization
		assert_noop!(Dao::remove_members(Origin::signed(*ALICE), org_id, *JOHN), Error::<Test>::NotMember);

	});
}

#[test]
fn user_can_view_organization_it_belongs_to_member_of() {
	new_test_ext().execute_with(|| {

		// Create Static Organization name, description, vision
		const ORG_NAME1: &'static [u8] = &[10];
		const ORG_DESC1: &'static [u8] = &[10];
		const ORG_VISION1: &'static [u8] = &[10];
		const ORG_NAME2: &'static [u8] = &[1];
		const ORG_DESC2: &'static [u8] = &[1];
		const ORG_VISION2: &'static [u8] = &[1];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(*ALICE), ORG_NAME1.to_vec(),
		ORG_DESC1.to_vec(), ORG_VISION1.to_vec()));

		// Ensure second organization can be created by a different user
		assert_ok!(Dao::create_organization(Origin::signed(*BOB), ORG_NAME2.to_vec(),
		ORG_DESC2.to_vec(), ORG_VISION2.to_vec()));

		let org_id_1 = Dao::organization_at_index(1).expect("must exist one org");
		let org_id_2 = Dao::organization_at_index(2).expect("must exist one org");
		// Ensure each organization was created successfully
		assert_eq!(Dao::members(org_id_1).len(), 1);
		assert_eq!(Dao::members(org_id_2).len(), 1);

		// Ensure users can be added to a DAO
		assert_ok!(Dao::add_members(Origin::signed(*ALICE), org_id_1, *EVE));
		assert_ok!(Dao::add_members(Origin::signed(*BOB), org_id_2, *EVE));

		// Ensure EVE belongs to two organizations
		assert_eq!(Dao::member_of(*EVE).len(), 2);

	});
}

#[test]
fn user_can_be_removed_from_organization_it_belongs_to_member_of() {
	new_test_ext().execute_with(|| {

		// Create Static Organization name, description, vision
		const ORG_NAME1: &'static [u8] = &[10];
		const ORG_DESC1: &'static [u8] = &[10];
		const ORG_VISION1: &'static [u8] = &[10];
		const ORG_NAME2: &'static [u8] = &[1];
		const ORG_DESC2: &'static [u8] = &[1];
		const ORG_VISION2: &'static [u8] = &[1];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(*ALICE), ORG_NAME1.to_vec(),
		ORG_DESC1.to_vec(), ORG_VISION1.to_vec()));

		// Ensure second organization can be created by a different user
		assert_ok!(Dao::create_organization(Origin::signed(*BOB), ORG_NAME2.to_vec(),
		ORG_DESC2.to_vec(), ORG_VISION2.to_vec()));

		let org_id_1 = Dao::organization_at_index(1).expect("must exist one org");
		let org_id_2 = Dao::organization_at_index(2).expect("must exist one org");
		// Ensure each organization was created successfully
		assert_eq!(Dao::members(org_id_1).len(), 1);
		assert_eq!(Dao::members(org_id_2).len(), 1);
		// Ensure user 4 is member of 0 organizations
		assert_eq!(Dao::member_of(*EVE).len(), 0);

		// Ensure user 4 can be added to a DAO
		assert_ok!(Dao::add_members(Origin::signed(*ALICE), org_id_1, *EVE));
		assert_ok!(Dao::add_members(Origin::signed(*BOB), org_id_2, *EVE));

		// Ensure the user 4 is member of 2 organizations
		assert_eq!(Dao::member_of(*EVE).len(), 2);

		// User can be removed from organization
		assert_ok!(Dao::remove_members(Origin::signed(*ALICE), org_id_1, *EVE));

		// Ensure user 4 belongs to 1 organizations
		assert_eq!(Dao::member_of(*EVE).len(), 1);

	});
}

#[test]
fn user_can_not_be_removed_from_organization_that_does_not_exist() {
	new_test_ext().execute_with(|| {

		// Create Static Organization name, description, vision
		const ORG_NAME1: &'static [u8] = &[10];
		const ORG_DESC1: &'static [u8] = &[10];
		const ORG_VISION1: &'static [u8] = &[10];
		const ORG_NAME2: &'static [u8] = &[1];
		const ORG_DESC2: &'static [u8] = &[1];
		const ORG_VISION2: &'static [u8] = &[1];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(*ALICE), ORG_NAME1.to_vec(),
		ORG_DESC1.to_vec(), ORG_VISION1.to_vec()));

		// Ensure second organization can be created by a different user
		assert_ok!(Dao::create_organization(Origin::signed(*BOB), ORG_NAME2.to_vec(),
		ORG_DESC2.to_vec(), ORG_VISION2.to_vec()));

		let org_id_1 = Dao::organization_at_index(1).expect("must exist one org");
		let org_id_2 = Dao::organization_at_index(2).expect("must exist one org");
		// Ensure each organization was created successfully
		assert_eq!(Dao::members(org_id_1).len(), 1);
		assert_eq!(Dao::members(org_id_2).len(), 1);
		// Ensure user EVE is member of 0 organizations
		assert_eq!(Dao::member_of(*EVE).len(), 0);

		// Ensure user EVE can be added to 1 organizations
		assert_ok!(Dao::add_members(Origin::signed(*ALICE), org_id_1, *EVE));

		// Ensure the user EVE is member of 1 organizations
		assert_eq!(Dao::member_of(*EVE).len(), 1);

		let hash = sp_core::H256::zero();
		// Throws error when attempting to remove user from non-existing organization
		assert_noop!(Dao::remove_members(Origin::signed(*BOB), hash, *EVE), Error::<Test>::InvalidOrganization);

		// Ensure user EVE belongs to 1 organizations
		assert_eq!(Dao::member_of(*EVE).len(), 1);

	});
}

// < -------- Integration Tests ------------->

#[test]
fn can_add_tasks_to_organization() {
	new_test_ext().execute_with(|| {

		let hash = sp_core::H256::zero();

		// Create Static Organization name, description, vision
		const ORG_NAME: &'static [u8] = &[10];
		const ORG_DESC: &'static [u8] = &[10];
		const ORG_VISION: &'static [u8] = &[10];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(*ALICE), ORG_NAME.to_vec(),
		ORG_DESC.to_vec(), ORG_VISION.to_vec()));

		let org_id = Dao::organization_at_index(1).expect("must exist one org");

		// Ensure tasks can be added to a DAO
		assert_ok!(Dao::add_tasks(Origin::signed(*ALICE), org_id, hash));

		// Ensure the organization has 1 task
		assert_eq!(Dao::organization_tasks(org_id).len(), 1);

	});
}

#[test]
fn can_add_task_to_organization_only_once() {
	new_test_ext().execute_with(|| {

		let hash = sp_core::H256::zero();

		// Create Static Organization name, description, vision
		const ORG_NAME: &'static [u8] = &[10];
		const ORG_DESC: &'static [u8] = &[10];
		const ORG_VISION: &'static [u8] = &[10];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(*ALICE), ORG_NAME.to_vec(),
		ORG_DESC.to_vec(), ORG_VISION.to_vec()));

		let org_id = Dao::organization_at_index(1).expect("must exist one org");

		// Add task twice
		assert_ok!(Dao::add_tasks(Origin::signed(*ALICE), org_id, hash));

		// Ensure Error is thrown
		assert_noop!(Dao::add_tasks(Origin::signed(*ALICE), org_id, hash), Error::<Test>::TaskAlreadyExists);

		// Check only 1 task was added
		assert_eq!(Dao::organization_tasks(org_id).len(), 1);

	});
}

#[test]
fn only_creator_can_add_task_to_organization() {
	new_test_ext().execute_with(|| {

		let hash = sp_core::H256::zero();
		// Create Static Organization name, description, vision
		const ORG_NAME: &'static [u8] = &[10];
		const ORG_DESC: &'static [u8] = &[10];
		const ORG_VISION: &'static [u8] = &[10];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(*ALICE), ORG_NAME.to_vec(),
		ORG_DESC.to_vec(), ORG_VISION.to_vec()));

		let org_id = Dao::organization_at_index(1).expect("must exist one org");

		// Throw error if another than Creator is trying to add members
		assert_noop!(Dao::add_tasks(Origin::signed(*BOB), org_id, hash), Error::<Test>::NotOrganizationCreator);
	});
}

#[test]
fn can_not_add_tasks_to_organization_that_does_not_exist() {
	new_test_ext().execute_with(|| {

		// Create zero hash
		let hash = sp_core::H256::zero();

		// Throw error if organization is not found
		assert_noop!(Dao::add_tasks(Origin::signed(*BOB), hash, hash), Error::<Test>::InvalidOrganization);
	});
}

#[test]
fn can_remove_task_from_organization() {
	new_test_ext().execute_with(|| {

		let hash = sp_core::H256::zero();
		// Create Static Organization name, description, vision
		const ORG_NAME: &'static [u8] = &[10];
		const ORG_DESC: &'static [u8] = &[10];
		const ORG_VISION: &'static [u8] = &[10];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(*ALICE), ORG_NAME.to_vec(),
		ORG_DESC.to_vec(), ORG_VISION.to_vec()));

		let org_id = Dao::organization_at_index(1).expect("must exist one org");

		// Add task to organization
		assert_ok!(Dao::add_tasks(Origin::signed(*ALICE), org_id, hash));

		// Check only 1 task was added
		assert_eq!(Dao::organization_tasks(org_id).len(), 1);

		// Remove task from organization
		assert_ok!(Dao::remove_tasks(Origin::signed(*ALICE), org_id, hash));

		// Ensure the organization tasks are 0
		assert_eq!(Dao::organization_tasks(org_id).len(), 0);

	});
}

#[test]
fn can_remove_task_from_organization_only_once() {
	new_test_ext().execute_with(|| {

		let hash = sp_core::H256::zero();

		// Create Static Organization name, description, vision
		const ORG_NAME: &'static [u8] = &[10];
		const ORG_DESC: &'static [u8] = &[10];
		const ORG_VISION: &'static [u8] = &[10];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(*ALICE), ORG_NAME.to_vec(),
		ORG_DESC.to_vec(), ORG_VISION.to_vec()));

		let org_id = Dao::organization_at_index(1).expect("must exist one org");

		// Add task to organization
		assert_ok!(Dao::add_tasks(Origin::signed(*ALICE), org_id, hash));

		// Remove task from organization
		assert_ok!(Dao::remove_tasks(Origin::signed(*ALICE), org_id, hash));

		// Ensure once the task has been removed, error is thrown
		assert_noop!(Dao::remove_tasks(Origin::signed(*ALICE), org_id, hash), Error::<Test>::TaskNotExist);

	});
}

#[test]
fn only_creator_can_remove_task_to_organization() {
	new_test_ext().execute_with(|| {

		let hash = sp_core::H256::zero();
		// Create Static Organization name, description, vision
		const ORG_NAME: &'static [u8] = &[10];
		const ORG_DESC: &'static [u8] = &[10];
		const ORG_VISION: &'static [u8] = &[10];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(*ALICE), ORG_NAME.to_vec(),
		ORG_DESC.to_vec(), ORG_VISION.to_vec()));

		let org_id = Dao::organization_at_index(1).expect("must exist one org");

		// Throw error if another than Creator is trying to remove members
		assert_noop!(Dao::remove_tasks(Origin::signed(*BOB), org_id, hash), Error::<Test>::NotOrganizationCreator);
	});
}

#[test]
fn can_not_remove_tasks_from_organization_that_does_not_exist() {
	new_test_ext().execute_with(|| {

		// Create zero hash
		let hash = sp_core::H256::zero();

		// Throw error if organization is not found
		assert_noop!(Dao::remove_tasks(Origin::signed(*BOB), hash, hash), Error::<Test>::InvalidOrganization);
	});
}
