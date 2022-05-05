// This file is part of Substrate.

// Copyright (C) 2022 UNIVERSALDOT FOUNDATION.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


//! # DAO Pallet
//!
//! - [`Config`]
//! - [`Pallet`]
//!
//! ## Overview
//!
//! Organizes People with a common Vision to work on projects.
//! This module works as an extension to the Task module since
//! it enables the creation of large projects which collect many tasks.
//!
//! A visionary user is able to propose a Vision for the future.
//! Within the vision, a specified Road-map is create that is broken
//! down into tasks. Thus a DAO is a collection of tasks who are undertaken
//! by people that believe in the vision of the Founder.
//!
//! Users support a Vision by signing a vision document. Signing a vision document enables
//! users to be added to a DAO where they will be able to create/fulfill tasks in
//! support of the overall vision.
//!
//! For completion of tasks, users are rewarded tokens and increased reputation.
//!
//! ## Interface
//!
//! ### Public Functions
//!
//! - `create_vision` - Function used to create vision of the future.
//!
//! - `remove_vision` - Function used to remove existing vision.
//!
//! - `sign_vision` - Function used to sign user to a vision. Signing a vision
//! indicates interest that the user are interested in creating said vision.
//!
//! - `unsign_vision` - Function used to unsign user from a vision. Unsigning a vision
//! indicates that a user is no longer interested in creating said vision.
//!
//! - `create_organization` - Function used to create a DAO organization.
//!
//! - `add_members` - Function used for a visionary to add members to his organization.
//!
//! - `remove_members` - Function used for a visionary to remove members from his organization.
//!
//! - `dissolve_organization` - Function used for a visionary to dissolve his organization.
//!
//! - `add_tasks` - Function used for a visionary to add tasks to his organization.
//!
//! - `remove_tasks` - Function used for a visionary to remove tasks from his organization.
//!
//! ## Related Modules
//!


#![cfg_attr(not(feature = "std"), no_std)]


pub use pallet::*;


#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use frame_support::sp_runtime::traits::Hash;

	use sp_std::vec::Vec;
	use scale_info::TypeInfo;
	use crate::weights::WeightInfo;

	// Account used in Dao Struct
	type AccountOf<T> = <T as frame_system::Config>::AccountId;

	// Struct for holding Dao information.
	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Dao<T: Config> {
		pub name: Vec<u8>,
		pub description: Vec<u8>,
		pub owner: AccountOf<T>,
		pub vision: Vec<u8>,
		pub created_time: <T as frame_system::Config>::BlockNumber,
		pub last_updated: <T as frame_system::Config>::BlockNumber,
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// WeightInfo provider.
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn vision_count)]
	/// VisionCount: Get total number of submitted Visions in the system
	pub(super) type VisionCount<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn vision)]
	/// Store Vision document in StorageMap as Vector with value: AccountID, BlockNumber
	pub(super) type Vision<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber), ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn organizations)]
	/// Storage for organizations data, key: hash of Dao struct, Value Dao struct.
	pub(super) type Organizations<T: Config> = StorageMap<_, Twox64Concat, T::Hash, Dao<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn members)]
	/// Create members of organization storage map with key: Hash and value: Vec<AccountID>
	pub(super) type Members<T: Config> = StorageMap<_, Twox64Concat, T::Hash, Vec<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn organization_count)]
	/// OrganizationCount: Get total number of organizations in the system
	pub(super) type OrganizationCount<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn organization_tasks)]
	/// Create organization storage map with key: hash of task and value: Vec<Hash of task>
	pub(super) type OrganizationTasks<T: Config> = StorageMap<_, Twox64Concat, T::Hash, Vec<T::Hash>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn member_of)]
	/// Storage item that indicates which DAO's a user belongs to [AccountID, Vec]
	pub(super) type MemberOf<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, Vec<T::Hash>, ValueQuery>;


	#[pallet::storage]
	#[pallet::getter(fn applicants_to_organization)]
	/// Storage Map to indicate which user agree with a proposed Vision [Vision, Vec[Account]]
	pub(super) type ApplicantsToOrganization<T: Config> = StorageMap<_, Twox64Concat, Vec<u8>, Vec<T::AccountId>, ValueQuery>;


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Vision successfully created [AccountID, Vec]
		VisionCreated(T::AccountId, Vec<u8>),

		/// Vision removed [AccountID, Vec]
		VisionRemoved(T::AccountId, Vec<u8>),

		/// Vision signed [AccountID, Vec]
		VisionSigned(T::AccountId, Vec<u8>),

		/// Vision signed [AccountID, Vec]
		VisionUnsigned(T::AccountId, Vec<u8>),

		/// DAO Organization was created [AccountID, DAO ID]
		OrganizationCreated(T::AccountId, T::Hash),

		/// DAO Organization was dissolved [AccountID, DAO ID]
		OrganizationDissolved(T::AccountId, T::Hash),

		/// Member has been added to an organization [AccountID, AccountID, DAO ID]
		MemberAdded(T::AccountId, T::AccountId, T::Hash),

		/// Member removed from an organization [AccountID, AccountID, DAO ID]
		MemberRemoved(T::AccountId, T::AccountId, T::Hash),

		/// Task added to an organization [AccountID, Task Hash, DAO ID]
		TaskAdded(T::AccountId, T::Hash, T::Hash),

		/// Task removed from an organization [AccountID, Task Hash, DAO ID]
		TaskRemoved(T::AccountId, T::Hash, T::Hash),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// The vision has already been created.
		VisionAlreadyExists,
		/// The Vision doesn't exist.
		NoSuchVision,
		/// You are not the owner of the vision.
		NotVisionOwner,
		/// Max limit for Visions reached.
		VisionCountOverflow,
		/// Max limit for Organizations reached.
		OrganizationCountOverflow,
		/// This vision has already been signed.
		AlreadySigned,
		/// You can't unsign from vision that that you haven't signed.
		NotSigned,
		/// No rights to remove. Only creator can remove an organization
		NotOrganizationCreator,
		/// User is already a member of this DAO.
		AlreadyMember,
		/// The organization doesn't exist.
		InvalidOrganization,
		/// The organization already exists.
		OrganizationAlreadyExists,
		/// The user is not a member of this organization.
		NotMember,
		/// Task doesn't exist.
		TaskNotExist,
		/// Task has been already added to organization.
		TaskAlreadyExists,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		/// Function for creating a vision and publishing it on chain [origin, vision]
		#[pallet::weight(<T as Config>::WeightInfo::create_vision(0))]
		pub fn create_vision(origin: OriginFor<T>, vision_document: Vec<u8>) -> DispatchResult {

			// Check that the extrinsic was signed and get the signer.
			let sender = ensure_signed(origin)?;

			// Verify that the specified vision has not already been created.
			ensure!(!Vision::<T>::contains_key(&vision_document), Error::<T>::VisionAlreadyExists);

			// Get the block number from the FRAME System pallet.
			let current_block = <frame_system::Pallet<T>>::block_number();

			// Store the vision with the sender and block number.
			Vision::<T>::insert(&vision_document, (&sender, current_block));

			//Increase Vision Count storage
			let new_count = Self::vision_count().checked_add(1).ok_or(<Error<T>>::VisionCountOverflow)?;
			<VisionCount<T>>::put(new_count);

			// Emit an event that the claim was created.
			Self::deposit_event(Event::VisionCreated(sender, vision_document));

			Ok(())
		}

		/// Function for removing a vision document [origin, vision]
		#[pallet::weight(<T as Config>::WeightInfo::remove_vision(0))]
        pub fn remove_vision(origin: OriginFor<T>, vision_document: Vec<u8>) -> DispatchResult {

			// Check that the extrinsic was signed and get the signer.
            let sender = ensure_signed(origin)?;

            // Verify that the specified vision has been created.
            ensure!(Vision::<T>::contains_key(&vision_document), Error::<T>::NoSuchVision);

            // Get owner of the vision.
            let (owner, _) = Vision::<T>::get(&vision_document);

            // Verify that sender of the current call is the vision creator
            ensure!(sender == owner, Error::<T>::NotVisionOwner);

            // Remove vision from storage.
            Vision::<T>::remove(&vision_document);

			// Reduce vision count
			let new_count = Self::vision_count().saturating_sub(1);
			<VisionCount<T>>::put(new_count);

            // Emit an event that the vision was erased.
            Self::deposit_event(Event::VisionRemoved(sender, vision_document));

            Ok(())
        }


		/// Function for signing a vision document [origin, vision]
		#[pallet::weight(<T as Config>::WeightInfo::sign_vision(0))]
		pub fn sign_vision(origin: OriginFor<T>, vision_document: Vec<u8>) -> DispatchResult {

			// Check that the extrinsic was signed and get the signer.
			let who = ensure_signed(origin)?;

			Self::member_signs_vision(&who, &vision_document)?;

			// Emit an event.
			Self::deposit_event(Event::VisionSigned(who, vision_document));

			Ok(())
		}

		/// Function for unsigning a vision document [origin, vision]
		#[pallet::weight(<T as Config>::WeightInfo::unsign_vision(0))]
		pub fn unsign_vision(origin: OriginFor<T>, vision_document: Vec<u8>) -> DispatchResult {

			// Check that the extrinsic was signed and get the signer.
			let who = ensure_signed(origin)?;

			Self::member_unsigns_vision(&who, &vision_document)?;

			// Emit an event.
			Self::deposit_event(Event::VisionUnsigned(who, vision_document));

			Ok(())
		}

		/// Function for creating an organization [origin, name, description, vision]
		#[pallet::weight(<T as Config>::WeightInfo::create_organization(0))]
		pub fn create_organization(origin: OriginFor<T>, name: Vec<u8>, description: Vec<u8>, vision: Vec<u8>) -> DispatchResult {

			// Check that the extrinsic was signed and get the signer.
			let who = ensure_signed(origin)?;

			//TODO: Ensure only visionary can crate DAOs

			// call public function to create org
			let org_id = Self::new_org(&who, &name, &description, &vision)?;

			// Emit an event.
			Self::deposit_event(Event::OrganizationCreated(who, org_id));

			Ok(())
		}

		/// Function for adding member to an organization [origin, org_id, AccountID]
		#[pallet::weight(<T as Config>::WeightInfo::add_members(0))]
		pub fn add_members(origin: OriginFor<T>, org_id: T::Hash, account: T::AccountId) -> DispatchResult {

			// Check that the extrinsic was signed and get the signer.
			let who = ensure_signed(origin)?;

			// call function to add member to organization
			Self::add_member_to_organization(&who, org_id, &account)?;

			// Emit an event.
			Self::deposit_event(Event::MemberAdded(who, account, org_id));

			Ok(())
		}

		/// Function for adding tasks to an organization [origin, org_id, task_hash]
		#[pallet::weight(<T as Config>::WeightInfo::add_tasks(0))]
		pub fn add_tasks(origin: OriginFor<T>, org_id: T::Hash, task: T::Hash) -> DispatchResult {

			// Check that the extrinsic was signed and get the signer.
			let who = ensure_signed(origin)?;

			// call function to add task to organization
			Self::add_task_to_organization(&who, org_id, &task)?;

			// Emit an event.
			Self::deposit_event(Event::TaskAdded(who, task, org_id));

			Ok(())
		}

		/// Function for removing member from an organization [origin, org_id, AccountID]
		#[pallet::weight(<T as Config>::WeightInfo::remove_members(0))]
		pub fn remove_members(origin: OriginFor<T>, org_id: T::Hash, account: T::AccountId) -> DispatchResult {

			// Check that the extrinsic was signed and get the signer.
			let who = ensure_signed(origin)?;

			// call function to remove member from organization
			Self::remove_member_from_organization(&who, org_id, &account)?;

			// Emit an event.
			Self::deposit_event(Event::MemberRemoved(who, account, org_id));

			Ok(())
		}

		/// Function for removing tasks from an organization [origin, org_id, task_hash]
		#[pallet::weight(<T as Config>::WeightInfo::remove_tasks(0))]
		pub fn remove_tasks(origin: OriginFor<T>, org_id: T::Hash, task: T::Hash) -> DispatchResult {

			// Check that the extrinsic was signed and get the signer.
			let who = ensure_signed(origin)?;

			// call function to add task to organization
			Self::remove_task_from_organization(&who, org_id, &task)?;

			// Emit an event.
			Self::deposit_event(Event::TaskRemoved(who, task, org_id));

			Ok(())
		}

		/// Function for dissolving an organization [origin, org_id]
		#[pallet::weight(<T as Config>::WeightInfo::dissolve_organization(0))]
		pub fn dissolve_organization(origin: OriginFor<T>, org_id: T::Hash) -> DispatchResult {

			// Check that the extrinsic was signed and get the signer.
			let who = ensure_signed(origin)?;

			// call function to remove organization
			Self::remove_org(&who, org_id)?;

			// Emit an event.
			Self::deposit_event(Event::OrganizationDissolved(who, org_id));

			Ok(())
		}

	}

	// *** Helper functions *** //
	impl<T:Config> Pallet<T> {
		// helper method for testing
		pub fn get_hash_for_dao(from_initiator: &T::AccountId, name: &[u8], description: &[u8], vision: &[u8], created_time: T::BlockNumber , last_updated: T::BlockNumber) -> T::Hash {
			let dao = Dao::<T> {
				name: name.to_vec(),
				description: description.to_vec(),
				owner: from_initiator.clone(),
				vision: vision.to_vec(),
				created_time,
				last_updated,
			};

			let hash = T::Hashing::hash_of(&dao);
			hash
		}
		pub fn new_org(from_initiator: &T::AccountId, name: &[u8], description: &[u8], vision: &[u8]) -> Result<T::Hash, DispatchError> {
			let current_block = <frame_system::Pallet<T>>::block_number();
			let dao = Dao::<T> {
				name: name.to_vec(),
				description: description.to_vec(),
				owner: from_initiator.clone(),
				vision: vision.to_vec(),
				created_time: current_block,
				last_updated: current_block,
			};

			let org_id = T::Hashing::hash_of(&dao);
			ensure!(<Organizations<T>>::get(org_id) == None, <Error<T>>::OrganizationAlreadyExists);

			// Insert Dao struct in Organizations storage
			<Organizations<T>>::insert(org_id, dao);

			// Insert vector into Hashmap
			<Members<T>>::insert(org_id, vec![from_initiator]);

			// Increase organization count
			let new_count =
				Self::organization_count().checked_add(1).ok_or(<Error<T>>::OrganizationCountOverflow)?;
			<OrganizationCount<T>>::put(new_count);
			Ok(org_id)
		}

		pub fn remove_org(from_initiator: &T::AccountId, org_id : T::Hash) -> Result<(), DispatchError> {

			// check if its DAO original creator
			Self::is_dao_founder(from_initiator, org_id)?;

			// Remove Dao struct from Organizations storage
			<Organizations<T>>::remove(org_id);
			// Remove organizational instance
			<Members<T>>::remove(org_id);

			// Reduce organization count
			let new_count = Self::organization_count().saturating_sub(1);
			<OrganizationCount<T>>::put(new_count);

			Ok(())
		}

		pub fn add_member_to_organization(from_initiator: &T::AccountId, org_id: T::Hash, account: &T::AccountId ) -> Result<(), DispatchError> {
			// Check if organization exists
			let mut members = Self::members(org_id);
			ensure!(!members.is_empty() , Error::<T>::InvalidOrganization);

			// check if its DAO original creator
			Self::is_dao_founder(from_initiator, org_id)?;

			// Check if already a member
			ensure!(!members.contains(account), <Error<T>>::AlreadyMember);

			// Insert account into organization
			members.push(account.clone());
			<Members<T>>::insert(org_id, &members);

			// Insert organizations into MemberOf
			let mut organizations = Self::member_of(&account);
			organizations.push(org_id);
			<MemberOf<T>>::insert(&account, organizations);

			Ok(())
		}

		pub fn add_task_to_organization(from_initiator: &T::AccountId, org_id: T::Hash, task: &T::Hash ) -> Result<(), DispatchError> {
			// Check if organization exists
			let members = Self::members(org_id);
			ensure!(!members.is_empty() , Error::<T>::InvalidOrganization);

			// check if its DAO original creator
			Self::is_dao_founder(from_initiator, org_id)?;

			// Check if already contains the task
			let mut tasks = Self::organization_tasks(org_id);
			ensure!(!tasks.contains(task), <Error<T>>::TaskAlreadyExists);

			// Insert task into organization
			tasks.push(*task);
			<OrganizationTasks<T>>::insert(org_id, &tasks);


			Ok(())
		}

		pub fn remove_member_from_organization(from_initiator: &T::AccountId, org_id: T::Hash, account: &T::AccountId ) -> Result<(), DispatchError> {
			// Check if organization exists
			let mut members = <Pallet<T>>::members(org_id);
			ensure!(!members.is_empty() , Error::<T>::InvalidOrganization);

			// check if its DAO original creator
			Self::is_dao_founder(from_initiator, org_id)?;

			// Find member and remove from Vector
			ensure!( members.iter().any(|a| *a == *account), Error::<T>::NotMember);
			members = members.into_iter().filter(|a| *a != *account).collect();
			// Update Organization Members
			<Members<T>>::insert(org_id, members);

			// Find current organizations and remove org_id from MemberOf user
			let mut current_organizations = <Pallet<T>>::member_of(&account);
			ensure!(current_organizations.iter().any(|a| *a == org_id), Error::<T>::InvalidOrganization);
			current_organizations = current_organizations.into_iter().filter(|a| *a !=
				org_id).collect();
			// Update MemberOf
			<MemberOf<T>>::insert(&account, &current_organizations);

			Ok(())
		}

		pub fn remove_task_from_organization(from_initiator: &T::AccountId, org_id: T::Hash, task: &T::Hash ) -> Result<(), DispatchError> {
			// Check if organization exists
			let member = <Pallet<T>>::members(org_id);
			ensure!(!member.is_empty() , Error::<T>::InvalidOrganization);

			// check if its DAO original creator
			Self::is_dao_founder(from_initiator, org_id)?;

			// Find task and remove from Vector
			let mut tasks = <Pallet<T>>::organization_tasks(org_id);
			ensure!(tasks.iter().any(|a| *a == *task), Error::<T>::TaskNotExist);
			tasks = tasks.into_iter().filter(|a| *a != *task).collect();

			// Update organization tasks
			<OrganizationTasks<T>>::insert(org_id, tasks);

			Ok(())
		}

		pub fn member_signs_vision(from_initiator: &T::AccountId, vision_document: &[u8]) -> Result<(), DispatchError> {

			// Verify that the specified vision has been created.
            ensure!(Vision::<T>::contains_key(vision_document), Error::<T>::NoSuchVision);

			// TODO: Perhaps use vision Hash instead of vision document
			// let hash_vision = T::Hashing::hash_of(&vision_document);

			let mut members = <Pallet<T>>::applicants_to_organization(vision_document);

			// Ensure not signed already
			ensure!(!members.contains(from_initiator), <Error<T>>::AlreadySigned);
			members.push(from_initiator.clone());

			// Update storage.
			<ApplicantsToOrganization<T>>::insert(vision_document, members);

			Ok(())
		}

		pub fn member_unsigns_vision(from_initiator: &T::AccountId, vision_document: &[u8]) -> Result<(), DispatchError> {

			// Verify that the specified vision has been created.
            ensure!(Vision::<T>::contains_key(vision_document), Error::<T>::NoSuchVision);

			// TODO: Perhaps use vision Hash instead of vision document
			// let hash_vision = T::Hashing::hash_of(&vision_document);

			let mut members = <Pallet<T>>::applicants_to_organization(vision_document);

			// Ensure not signed already
			ensure!(members.iter().any(|a| *a == *from_initiator), Error::<T>::NotSigned);
			members = members.into_iter().filter(|a| *a != *from_initiator).collect();

			// Update storage.
			<ApplicantsToOrganization<T>>::insert(vision_document, members);

			Ok(())
		}



		pub fn is_dao_founder(from_initiator: &T::AccountId, org_id: T::Hash) -> Result<bool,
		DispatchError> {
			let first_account = Self::members(org_id);
			if first_account[0] == *from_initiator {
				Ok(true)
			} else { Err(Error::<T>::NotOrganizationCreator.into()) }
		}
	}
}
