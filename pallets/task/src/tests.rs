use core::convert::TryInto;

use crate::TaskStatus;
use crate::{mock::*, Error};
use frame_support::traits::fungible::Inspect;
use frame_support::{assert_noop, assert_ok, traits::{UnixTime, Hooks}};

pub const USERNAME:[u8; 1] = [7];
pub const TITLE:[u8; 1] = [1];
pub const MILLISEC_PER_BLOCK: u64 = 12000; // 12 sec for a block
fn get_deadline() -> u64 {
		// deadline is current time + 1 hour
		let deadline = <Time as UnixTime>::now() + std::time::Duration::from_millis(3600 * 1000_u64);
		let deadline_u64 = deadline.as_secs() * 1000_u64;
		assert_eq!(deadline.as_millis(), deadline_u64 as u128);
		deadline_u64
}

fn run_to_block(n: u64) {
	Task::on_finalize(System::block_number());
	for b in (System::block_number() + 1)..=n {
		next_block(b);
		if b != n {
			Task::on_finalize(System::block_number());
		}
	}
}

fn next_block(n: u64) {
	Time::set_timestamp(MILLISEC_PER_BLOCK * n);
	System::set_block_number(n);
	Task::on_initialize(n);
}

#[test]
fn create_new_task(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), USERNAME.to_vec().try_into().unwrap(), Vec::new().try_into().unwrap()));

		let mut vec = Vec::new();
		vec.push(2);

		// Ensure new task can be created with [signer, specification, budget, deadline]
		assert_ok!(Task::create_task(Origin::signed(1), TITLE.to_vec(), vec, 7, get_deadline()));
	});
}

#[test]
fn fund_transfer_on_create_task(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), USERNAME.to_vec().try_into().unwrap(), Vec::new().try_into().unwrap()));

		let mut vec = Vec::new();
		vec.push(2);

		assert_eq!(Balances::balance(&1), 1000);
		// Ensure new task can be created with [signer, specification, budget, deadline]
		assert_ok!(Task::create_task(Origin::signed(1), TITLE.to_vec(), vec, 7, get_deadline()));
		assert_eq!(Balances::balance(&1), 993);
		let task_id = Task::tasks_owned(&1)[0];
		assert_eq!(Balances::balance(&Task::account_id(&task_id)), 7);
	});
}

#[test]
fn increase_task_count_when_creating_task(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), USERNAME.to_vec().try_into().unwrap(), Vec::new().try_into().unwrap()));

		let mut vec = Vec::new();
		vec.push(2);

		// Ensure new task can be created with [signer, specification, budget, deadline]
		assert_ok!(Task::create_task(Origin::signed(1), TITLE.to_vec(), vec, 7, get_deadline()));

		// Assert that count is incremented by 1 after task creation
		assert_eq!(Task::task_count(), 1);
	});
}

#[test]
fn increase_task_count_when_creating_two_tasks(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), USERNAME.to_vec().try_into().unwrap(), Vec::new().try_into().unwrap()));

		let mut vec1 = Vec::new();
		vec1.push(2);

		let mut vec2 = Vec::new();
		vec2.push(7);

		// Ensure new task can be created with [signer, specification, budget, deadline]
		assert_ok!(Task::create_task(Origin::signed(1), TITLE.to_vec(), vec1, 7, get_deadline()));
		assert_ok!(Task::create_task(Origin::signed(1), TITLE.to_vec(), vec2, 99, get_deadline()));

		// Assert that count is incremented to 2 after task creation
		assert_eq!(Task::task_count(), 2);
	});
}

#[test]
fn cant_own_more_tax_than_max_tasks(){
	new_test_ext().execute_with( || {

		// TODO: use MaxTasksOwned instead of hardcoded values;

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), USERNAME.to_vec().try_into().unwrap(), Vec::new().try_into().unwrap()));

		// Create 77 tasks  ExceedMaxTasksOwned
		for n in 0..77 {

			let mut vec1 = Vec::new();
			vec1.push(n);

			// Ensure new task can be created with [signer, specification, budget, deadline]
			assert_ok!(Task::create_task(Origin::signed(1), TITLE.to_vec(), vec1, 7, get_deadline()));
		}

		// Assert that count is incremented to 2 after task creation
		assert_eq!(Task::task_count(), 77);

		let mut vec2 = Vec::new();
		vec2.push(7);

		// Assert that when creating the 77 Task, Error is thrown
		assert_noop!(Task::create_task(Origin::signed(1), TITLE.to_vec(), vec2, 7, get_deadline()), Error::<Test>::ExceedMaxTasksOwned);

	});

}

#[test]
fn assign_task_to_current_owner(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(10), USERNAME.to_vec().try_into().unwrap(), Vec::new().try_into().unwrap()));

		let mut vec1 = Vec::new();
		vec1.push(2);

		assert_ok!(Task::create_task(Origin::signed(10), TITLE.to_vec(), vec1, 7, get_deadline()));

		// Get task through the hash
		let hash = Task::tasks_owned(10)[0];
		let task = Task::tasks(hash).expect("should found the task");

		assert_eq!(task.current_owner, 10);
	});
}

#[test]
fn verify_inputs_outputs_to_tasks(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(10), USERNAME.to_vec().try_into().unwrap(), Vec::new().try_into().unwrap()));

		let mut vec1 = Vec::new();
		vec1.push(2);

		assert_ok!(Task::create_task(Origin::signed(10), TITLE.to_vec(), vec1, 7, get_deadline()));

		// Get task through the hash
		let hash = Task::tasks_owned(10)[0];
		let task = Task::tasks(hash).expect("should found the task");

		// Ensure that task properties are assigned correctly
		assert_eq!(task.current_owner, 10);
		assert_eq!(task.budget, 7);
		assert_eq!(task.title, &[1]);
	});
}

#[test]
fn task_can_be_updated_after_it_is_created(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(10), USERNAME.to_vec().try_into().unwrap(), Vec::new().try_into().unwrap()));


		let mut vec1 = Vec::new();
		vec1.push(2);

		let vec2 = Vec::new();
		vec1.push(3);

		assert_ok!(Task::create_task(Origin::signed(10), TITLE.to_vec(), vec1, 1, get_deadline()));

		// Get task through the hash
		let hash = Task::tasks_owned(10)[0];
		let task = Task::tasks(hash).expect("should found the task");

		// assert the budget is correct
		assert_eq!(task.budget, 1);

		assert_ok!(Task::update_task(Origin::signed(10), hash, TITLE.to_vec(), vec2, 7, get_deadline()));

		// Get task through the hash
		let hash = Task::tasks_owned(10)[0];
		let task = Task::tasks(hash).expect("should found the task");

		// Ensure that task properties are assigned correctly
		assert_eq!(task.current_owner, 10);
		assert_eq!(task.budget, 7);
		assert_eq!(task.title, &[1]);
	});
}

#[test]
fn task_can_be_updated_only_by_one_who_created_it(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(10), USERNAME.to_vec().try_into().unwrap(), Vec::new().try_into().unwrap()));

		let mut vec1 = Vec::new();
		vec1.push(2);

		let vec2 = Vec::new();
		vec1.push(3);

		assert_ok!(Task::create_task(Origin::signed(10), TITLE.to_vec(), vec1, 1, get_deadline()));

		// Get task through the hash
		let hash = Task::tasks_owned(10)[0];

		// Throw error when someone other than creator tries to update task
		assert_noop!(Task::update_task(Origin::signed(7), hash, TITLE.to_vec(), vec2, 7, get_deadline()), Error::<Test>::OnlyInitiatorUpdatesTask);

	});
}

#[test]
fn task_can_be_updated_only_after_it_has_been_created(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(10), USERNAME.to_vec().try_into().unwrap(), Vec::new().try_into().unwrap()));

		let mut vec1 = Vec::new();
		vec1.push(2);

		let vec2 = Vec::new();
		vec1.push(3);

		// Ensure task can be created
		assert_ok!(Task::create_task(Origin::signed(10), TITLE.to_vec(), vec1, 1, get_deadline()));

		// Get task through the hash
		let hash = Task::tasks_owned(10)[0];

		// Ensure task is started by new current_owner (user 2)
		assert_ok!(Task::start_task(Origin::signed(2), hash));


		// Throw error when someone other than creator tries to update task
		assert_noop!(Task::update_task(Origin::signed(10), hash, TITLE.to_vec(), vec2, 7, get_deadline()), Error::<Test>::NoPermissionToUpdate);

	});
}

#[test]
fn start_tasks_assigns_new_current_owner(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), USERNAME.to_vec().try_into().unwrap(), Vec::new().try_into().unwrap()));
		assert_ok!(Profile::create_profile(Origin::signed(2), USERNAME.to_vec().try_into().unwrap(), Vec::new().try_into().unwrap()));

		let mut vec1 = Vec::new();
		vec1.push(2);

		// Ensure new task can be created with [signer, specification, budget]
		assert_ok!(Task::create_task(Origin::signed(1), TITLE.to_vec(), vec1, 7, get_deadline()));

		// Ensure new task is assigned to new current_owner (user 1)
		let hash = Task::tasks_owned(1)[0];
		let task = Task::tasks(hash).expect("should found the task");
		assert_eq!(task.current_owner, 1);
		assert_eq!(Task::tasks_owned(1).len(), 1);

		// Ensure task is started by new current_owner (user 2)
		assert_ok!(Task::start_task(Origin::signed(2), hash));

		// Ensure when task is started user1 has 0 tasks, and user2 has 1
		assert_eq!(Task::tasks_owned(1).len(), 0);
		assert_eq!(Task::tasks_owned(2).len(), 1);

	});
}

#[test]
fn start_tasks_assigns_task_to_volunteer(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), USERNAME.to_vec().try_into().unwrap(), Vec::new().try_into().unwrap()));
		let mut vec1 = Vec::new();
		vec1.push(2);

		// Ensure new task can be created with [signer, specification, budget]
		assert_ok!(Task::create_task(Origin::signed(1), TITLE.to_vec(), vec1, 7, get_deadline()));

		// Ensure new task is assigned to new current_owner (user 1)
		let hash = Task::tasks_owned(1)[0];
		let task = Task::tasks(hash).expect("should found the task");
		assert_eq!(task.current_owner, 1);
		assert_eq!(Task::tasks_owned(1).len(), 1);

		// Ensure task is started by new current_owner (user 2)
		assert_ok!(Task::start_task(Origin::signed(2), hash));

		// Ensure when task is started it is assigned to volunteer (user 2)
		assert_eq!(task.volunteer, 1);
		assert_eq!(Task::tasks_owned(2).len(), 1);
		assert_eq!(Task::tasks_owned(1).len(), 0);
	});
}

#[test]
fn completing_tasks_assigns_new_current_owner(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), USERNAME.to_vec().try_into().unwrap(), Vec::new().try_into().unwrap()));

		let mut vec1 = Vec::new();
		vec1.push(2);

		// Ensure new task can be created with [signer, specification, budget, deadline]
		assert_ok!(Task::create_task(Origin::signed(1), TITLE.to_vec(), vec1, 7, get_deadline()));

		// Ensure new task is assigned to new current_owner (user 1)
		let hash = Task::tasks_owned(1)[0];
		let task = Task::tasks(hash).expect("should found the task");
		assert_eq!(task.current_owner, 1);
		assert_eq!(Task::tasks_owned(1).len(), 1);

		// Ensure task is started by new current_owner (user 2)
		assert_ok!(Task::start_task(Origin::signed(2), hash));

		// Ensure when task is started user1 has 0 tasks, and user2 has 1
		assert_eq!(Task::tasks_owned(1).len(), 0);
		assert_eq!(Task::tasks_owned(2).len(), 1);

		// Ensure task is completed by current current_owner (user 2)
		assert_ok!(Task::complete_task(Origin::signed(2), hash));

		// Ensure that the ownership is reversed again
		assert_eq!(task.current_owner, 1);
		assert_eq!(Task::tasks_owned(1).len(), 1);
		assert_eq!(Task::tasks_owned(2).len(), 0);
	});
}

#[test]
fn the_volunteer_is_different_from_task_creator(){
	new_test_ext().execute_with( || {

		assert_ok!(Profile::create_profile(Origin::signed(1), USERNAME.to_vec(), Vec::new()));

		let mut vec1 = Vec::new();
		vec1.push(2);

		// Ensure new task can be created with [signer, specification, budget, deadline]
		assert_ok!(Task::create_task(Origin::signed(1), TITLE.to_vec(), vec1, 7, get_deadline()));

		let hash = Task::tasks_owned(1)[0];
		assert_noop!(Task::start_task(Origin::signed(1), hash), Error::<Test>::NoPermissionToStart);

	});
}


#[test]
fn task_can_only_be_started_once(){
	new_test_ext().execute_with( || {

		assert_ok!(Profile::create_profile(Origin::signed(1), USERNAME.to_vec(), Vec::new()));

		let mut vec1 = Vec::new();
		vec1.push(2);

		// Ensure new task can be created with [signer, specification, budget, deadline]
		assert_ok!(Task::create_task(Origin::signed(1), TITLE.to_vec(), vec1, 7, get_deadline()));

		// Ensure that task can't be started once its started
		let hash = Task::tasks_owned(1)[0];
		assert_ok!(Task::start_task(Origin::signed(2), hash));
		assert_noop!(Task::start_task(Origin::signed(2), hash), Error::<Test>::NoPermissionToStart);

	});
}

#[test]
fn task_can_only_be_finished_by_the_user_who_started_it(){
	new_test_ext().execute_with( || {

		assert_ok!(Profile::create_profile(Origin::signed(1), USERNAME.to_vec(), Vec::new()));

		let mut vec1 = Vec::new();
		vec1.push(2);

		// Ensure new task can be created with [signer, specification, budget, deadline]
		assert_ok!(Task::create_task(Origin::signed(1), TITLE.to_vec(), vec1, 7, get_deadline()));

		// Ensure that task can't be started once its started
		let hash = Task::tasks_owned(1)[0];
		assert_ok!(Task::start_task(Origin::signed(2), hash));
		
		// Ensure that a user who didn't start the task has no permission to complete it
		assert_noop!(Task::complete_task(Origin::signed(1), hash), Error::<Test>::NoPermissionToComplete);

	});
}

#[test]
fn task_can_be_removed_by_owner(){
	new_test_ext().execute_with( || {

		assert_ok!(Profile::create_profile(Origin::signed(1), USERNAME.to_vec(), Vec::new()));

		let mut vec1 = Vec::new();
		vec1.push(2);

		// Ensure new task can be created with [signer, specification, budget, deadline]
		assert_ok!(Task::create_task(Origin::signed(1), TITLE.to_vec(), vec1, 7, get_deadline()));

		// Ensure that task can't be started once its started
		let hash = Task::tasks_owned(1)[0];
		
		// Ensure another user can't remove the task
		assert_noop!(Task::remove_task(Origin::signed(2), hash), Error::<Test>::NoPermissionToRemove);
		
		// Ensure the task can be removed
		assert_ok!(Task::remove_task(Origin::signed(1), hash));
		assert_eq!(Task::task_count(), 0);

	});
}

#[test]
fn task_can_be_removed_only_when_status_is_created(){
	new_test_ext().execute_with( || {

		assert_ok!(Profile::create_profile(Origin::signed(1), USERNAME.to_vec(), Vec::new()));

		let mut vec1 = Vec::new();
		vec1.push(2);

		// Ensure new task can be created with [signer, specification, budget, deadline]
		assert_ok!(Task::create_task(Origin::signed(1), TITLE.to_vec(), vec1, 7, get_deadline()));

		// Ensure that task can't be started once its started
		let hash = Task::tasks_owned(1)[0];
		assert_ok!(Task::start_task(Origin::signed(2), hash));
		
		// Ensure another user can't remove the task
		assert_noop!(Task::remove_task(Origin::signed(2), hash), Error::<Test>::NoPermissionToRemove);
		
	});
}


#[test]
fn only_creator_accepts_task(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), USERNAME.to_vec().try_into().unwrap(), Vec::new().try_into().unwrap()));
		assert_ok!(Profile::create_profile(Origin::signed(2), USERNAME.to_vec().try_into().unwrap(), Vec::new().try_into().unwrap()));

		let mut vec1 = Vec::new();
		vec1.push(2);

		// Ensure new task can be created with [signer, specification, budget]
		assert_ok!(Task::create_task(Origin::signed(1), TITLE.to_vec(), vec1, 7, get_deadline()));

		// Ensure new task is assigned to new current_owner (user 1)
		let hash = Task::tasks_owned(1)[0];
		let task = Task::tasks(hash).expect("should found the task");
		assert_eq!(task.current_owner, 1);
		assert_eq!(Task::tasks_owned(1).len(), 1);

		// Ensure task is started by new current_owner (user 2)
		assert_ok!(Task::start_task(Origin::signed(2), hash));

		// Ensure when task is started user1 has 0 tasks, and user2 has 1
		assert_eq!(Task::tasks_owned(1).len(), 0);
		assert_eq!(Task::tasks_owned(2).len(), 1);

		// Ensure task is completed by current current_owner (user 2)
		assert_ok!(Task::complete_task(Origin::signed(2), hash));

		// Ensure that the ownership is reversed again
		assert_eq!(Task::tasks_owned(1).len(), 1);
		assert_eq!(Task::tasks_owned(2).len(), 0);

		// Ensure task is accepted by task creator (user 1)
		assert_noop!(Task::accept_task(Origin::signed(2), hash), Error::<Test>::OnlyInitiatorAcceptsTask);
		assert_ok!(Task::accept_task(Origin::signed(1), hash));
	});
}

#[test]
fn volunteer_gets_paid_on_task_completion(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), USERNAME.to_vec().try_into().unwrap(), Vec::new().try_into().unwrap()));
		assert_ok!(Profile::create_profile(Origin::signed(2), USERNAME.to_vec().try_into().unwrap(), Vec::new().try_into().unwrap()));

		let mut vec1 = Vec::new();
		vec1.push(2);

		// Ensure new task can be created with [signer, specification, budget]
		assert_ok!(Task::create_task(Origin::signed(1), TITLE.to_vec(), vec1, 7, get_deadline()));

		let hash = Task::tasks_owned(1)[0];
		// Ensure task is started by new current_owner (user 2)
		assert_ok!(Task::start_task(Origin::signed(2), hash));


		// Ensure task is completed by current current_owner (user 2)
		assert_ok!(Task::complete_task(Origin::signed(2), hash));

		// Ensure task is accepted by task creator (user 1)
		// User 2 gets fund for completing task after it is accepted by user 1
		assert_eq!(Balances::balance(&2), 1000);
		assert_ok!(Task::accept_task(Origin::signed(1), hash));
		assert_eq!(Balances::balance(&2), 1007);

	});
}

#[test]
fn only_started_task_can_be_completed(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), USERNAME.to_vec().try_into().unwrap(), Vec::new().try_into().unwrap()));
		assert_ok!(Profile::create_profile(Origin::signed(2), USERNAME.to_vec().try_into().unwrap(), Vec::new().try_into().unwrap()));

		let mut vec1 = Vec::new();
		vec1.push(2);

		// Ensure new task can be created with [signer, specification, budget, deadline]
		assert_ok!(Task::create_task(Origin::signed(1), TITLE.to_vec(), vec1, 7, get_deadline()));

		// Ensure new task is assigned to new current_owner (user 1)
		let hash = Task::tasks_owned(1)[0];
		let task = Task::tasks(hash).expect("should found the task");
		assert_eq!(task.current_owner, 1);
		assert_eq!(Task::tasks_owned(1).len(), 1);

		assert_noop!(Task::complete_task(Origin::signed(2), hash), Error::<Test>::NoPermissionToComplete);

		// Ensure task is started by new current_owner (user 2)
		assert_ok!(Task::start_task(Origin::signed(2), hash));

		// Ensure task is completed by current current_owner (user 2)
		assert_ok!(Task::complete_task(Origin::signed(2), hash));
	});
}

#[test]
fn when_task_is_accepted_ownership_is_cleared(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), USERNAME.to_vec().try_into().unwrap(), Vec::new().try_into().unwrap()));
		assert_ok!(Profile::create_profile(Origin::signed(2), USERNAME.to_vec().try_into().unwrap(), Vec::new().try_into().unwrap()));

		let mut vec1 = Vec::new();
		vec1.push(2);

		// Ensure new task can be created with [signer, specification, budget]
		assert_ok!(Task::create_task(Origin::signed(1), TITLE.to_vec(), vec1, 7, get_deadline()));

		// Ensure new task is assigned to new current_owner (user 1)
		let hash = Task::tasks_owned(1)[0];
		let task = Task::tasks(hash).expect("should found the task");
		assert_eq!(task.current_owner, 1);
		assert_eq!(Task::tasks_owned(1).len(), 1);

		// Ensure task is started by new current_owner (user 2)
		assert_ok!(Task::start_task(Origin::signed(2), hash));

		// Ensure when task is started user1 has 0 tasks, and user2 has 1
		assert_eq!(Task::tasks_owned(1).len(), 0);
		assert_eq!(Task::tasks_owned(2).len(), 1);

		// Ensure task is completed by current current_owner (user 2)
		assert_ok!(Task::complete_task(Origin::signed(2), hash));

		// Ensure that the ownership is reversed again
		assert_eq!(Task::tasks_owned(1).len(), 1);
		assert_eq!(Task::tasks_owned(2).len(), 0);

		// Ensure task is accepted by task creator (user 1)
		assert_ok!(Task::accept_task(Origin::signed(1), hash));

		// Ensure ownership of task is cleared
		assert_eq!(Task::tasks_owned(1).len(), 0);
		assert_eq!(Task::tasks_owned(2).len(), 0);
	});
}


#[test]
fn decrease_task_count_when_accepting_task(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), USERNAME.to_vec().try_into().unwrap(), Vec::new().try_into().unwrap()));

		let mut vec = Vec::new();
		vec.push(2);

		// Ensure new task can be created with [signer, specification, budget]
		assert_ok!(Task::create_task(Origin::signed(1), TITLE.to_vec(), vec, 8, get_deadline()));

		// Get hash of task owned
		let hash = Task::tasks_owned(1)[0];
		let _task = Task::tasks(hash).expect("should found the task");

		// Accepting task decreases count
		assert_ok!(Task::accept_task(Origin::signed(1), hash));
		assert_eq!(Task::task_count(), 0);
	});
}

#[test]
fn task_can_be_rejected_by_creator(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), USERNAME.to_vec().try_into().unwrap(), Vec::new().try_into().unwrap()));

		let mut vec = Vec::new();
		vec.push(2);

		// Ensure new task can be created with [signer, specification, budget]
		assert_ok!(Task::create_task(Origin::signed(1), TITLE.to_vec(), vec, 8, get_deadline()));

		// Get hash of task owned
		let hash = Task::tasks_owned(1)[0];
		let _task = Task::tasks(hash).expect("should found the task");

		// Ensure task is started by new current_owner (user 2)
		assert_ok!(Task::start_task(Origin::signed(2), hash));

		// Ensure when task is started user1 has 0 tasks, and user2 has 1
		assert_eq!(Task::tasks_owned(1).len(), 0);
		assert_eq!(Task::tasks_owned(2).len(), 1);

		// Ensure task is completed by current current_owner (user 2)
		assert_ok!(Task::complete_task(Origin::signed(2), hash));

		// Task is rejected by creator
		assert_ok!(Task::reject_task(Origin::signed(1), hash));

		// Assert that the status is back in progress and, owner is the volunteer
		let hash = Task::tasks_owned(2)[0];
		let task = Task::tasks(hash).expect("should found the task");
		assert_eq!(task.current_owner, 2);
		assert_eq!(task.status, TaskStatus::InProgress);

	});
}

// <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<  Integration tests  >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>


#[test]
fn increase_profile_reputation_when_task_completed(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), USERNAME.to_vec().try_into().unwrap(), Vec::new().try_into().unwrap()));
		assert_ok!(Profile::create_profile(Origin::signed(2), USERNAME.to_vec().try_into().unwrap(), Vec::new().try_into().unwrap()));

		let mut vec1 = Vec::new();
		vec1.push(2);

		// Ensure new task can be created with [signer, specification, budget]
		assert_ok!(Task::create_task(Origin::signed(1), TITLE.to_vec(), vec1, 7, get_deadline()));

		// Ensure new task is assigned to new current_owner (user 1)
		let hash = Task::tasks_owned(1)[0];
		let task = Task::tasks(hash).expect("should found the task");
		assert_eq!(task.current_owner, 1);

		// Ensure task is started by new current_owner (user 2)
		assert_ok!(Task::start_task(Origin::signed(2), hash));

		// Ensure task is completed by current current_owner (user 2)
		assert_ok!(Task::complete_task(Origin::signed(2), hash));

		// Ensure task is accepted by task creator (user 1)
		assert_ok!(Task::accept_task(Origin::signed(1), hash));

		let profile1 = Profile::profiles(1).expect("should find the profile");
		let profile2 = Profile::profiles(2).expect("should find the profile");

		// Ensure that the reputation has been added to both profiles
		assert_eq!(profile1.reputation, 1);
		assert_eq!(profile2.reputation, 1);

	});
}

#[test]
fn only_add_reputation_when_task_has_been_accepted(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), USERNAME.to_vec().try_into().unwrap(), Vec::new().try_into().unwrap()));

		let mut vec = Vec::new();
		vec.push(2);

		// Ensure new task can be created with [signer, specification, budget]
		assert_ok!(Task::create_task(Origin::signed(1), TITLE.to_vec(), vec, 8, get_deadline()));

		// Get hash of task owned
		let hash = Task::tasks_owned(1)[0];
		let _task = Task::tasks(hash).expect("should found the task");

		// Ensure task can be accepted
		assert_ok!(Task::accept_task(Origin::signed(1), hash));

		// Reputation should remain 0 since the task was removed without being completed
		// TODO: Make sure that user creating a task is not the same as the one completing it
		let profile = Profile::profiles(1).expect("should find the profile");
		assert_eq!(profile.reputation, 2);
	});
}

#[test]
fn delete_task_after_deadline() {
	new_test_ext().execute_with( || {
		run_to_block(1);
		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), USERNAME.to_vec().try_into().unwrap(), Vec::new().try_into().unwrap()));

		let mut vec = Vec::new();
		vec.push(2);

		// Ensure new task can be created with [signer, specification, budget]
		assert_ok!(Task::create_task(Origin::signed(1), TITLE.to_vec(), vec, 8, get_deadline()));
		let hash = Task::tasks_owned(1)[0];
		let task = Task::tasks(hash);
		assert!(task.is_some());
		// deadline is 1 hour => 3600 sec => 300 blocks as 12 secs per block
		run_to_block(302);
		let task = Task::tasks(hash);
		assert!(task.is_none());
	});
}

