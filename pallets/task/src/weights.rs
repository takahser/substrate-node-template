// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_task
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-04-25, STEPS: `100`, REPEAT: 50, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:

// ./target/release/node-template

// benchmark

// --chain

// dev

// --execution

// wasm

// --wasm-execution

// compiled

// --pallet

// pallet_task

// --extrinsic

// *

// --steps

// 100

// --repeat

// 50

// --output

// ./pallets/task/src/weights.rs

// --template

// .maintain/frame-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_task.
pub trait WeightInfo {
	
	fn create_task(s: u32, x: u32, ) -> Weight;
	
	fn update_task(s: u32, x: u32, ) -> Weight;
	
	fn start_task(s: u32, x: u32, ) -> Weight;
	
	fn complete_task(s: u32, x: u32, ) -> Weight;
	
	fn accept_task(s: u32, x: u32, ) -> Weight;
	
	fn reject_task(s: u32, x: u32, ) -> Weight;
	
}

/// Weights for pallet_task using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {

	
	
	// Storage: Profile Profiles (r:1 w:0)
	
	// Storage: Timestamp Now (r:1 w:0)
	
	// Storage: Task TasksOwned (r:1 w:1)
	
	// Storage: Task TaskCount (r:1 w:1)
	
	// Storage: Task Tasks (r:0 w:1)
	
	fn create_task(s: u32, _x: u32, ) -> Weight {
		(23_499_000 as Weight)
			
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(s as Weight))
			
			
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			
			
			
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			
			
	}
	
	
	// Storage: Task Tasks (r:1 w:1)
	
	// Storage: Profile Profiles (r:1 w:0)
	
	// Storage: Timestamp Now (r:1 w:0)
	
	fn update_task(_s: u32, _x: u32, ) -> Weight {
		(22_928_000 as Weight)
			
			
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			
			
			
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			
			
	}
	
	
	// Storage: Task Tasks (r:1 w:1)
	
	// Storage: Task TasksOwned (r:1 w:1)
	
	fn start_task(_s: u32, _x: u32, ) -> Weight {
		(21_900_000 as Weight)
			
			
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			
			
			
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			
			
	}
	
	
	// Storage: Task Tasks (r:1 w:1)
	
	// Storage: Task TasksOwned (r:1 w:1)
	
	fn complete_task(_s: u32, _x: u32, ) -> Weight {
		(22_149_000 as Weight)
			
			
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			
			
			
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			
			
	}
	
	
	// Storage: Task Tasks (r:1 w:1)
	
	// Storage: Task TasksOwned (r:1 w:1)
	
	// Storage: Task TaskCount (r:1 w:1)
	
	fn accept_task(_s: u32, _x: u32, ) -> Weight {
		(27_182_000 as Weight)
			
			
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			
			
			
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			
			
	}
	
	
	// Storage: Task Tasks (r:1 w:1)
	
	// Storage: Task TasksOwned (r:1 w:1)
	
	fn reject_task(_s: u32, _x: u32, ) -> Weight {
		(23_789_000 as Weight)
			
			
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			
			
			
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			
			
	}
	
}

// For backwards compatibility and tests
impl WeightInfo for () {
	
	
	// Storage: Profile Profiles (r:1 w:0)
	
	// Storage: Timestamp Now (r:1 w:0)
	
	// Storage: Task TasksOwned (r:1 w:1)
	
	// Storage: Task TaskCount (r:1 w:1)
	
	// Storage: Task Tasks (r:0 w:1)
	
	fn create_task(s: u32, _x: u32, ) -> Weight {
		(23_499_000 as Weight)
			
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(s as Weight))
			
			
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			
			
			
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
			
			
	}
	
	
	// Storage: Task Tasks (r:1 w:1)
	
	// Storage: Profile Profiles (r:1 w:0)
	
	// Storage: Timestamp Now (r:1 w:0)
	
	fn update_task(_s: u32, _x: u32, ) -> Weight {
		(22_928_000 as Weight)
			
			
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			
			
			
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
			
			
	}
	
	
	// Storage: Task Tasks (r:1 w:1)
	
	// Storage: Task TasksOwned (r:1 w:1)
	
	fn start_task(_s: u32, _x: u32, ) -> Weight {
		(21_900_000 as Weight)
			
			
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			
			
			
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
			
			
	}
	
	
	// Storage: Task Tasks (r:1 w:1)
	
	// Storage: Task TasksOwned (r:1 w:1)
	
	fn complete_task(_s: u32, _x: u32, ) -> Weight {
		(22_149_000 as Weight)
			
			
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			
			
			
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
			
			
	}
	
	
	// Storage: Task Tasks (r:1 w:1)
	
	// Storage: Task TasksOwned (r:1 w:1)
	
	// Storage: Task TaskCount (r:1 w:1)
	
	fn accept_task(_s: u32, _x: u32, ) -> Weight {
		(27_182_000 as Weight)
			
			
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			
			
			
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
			
			
	}
	
	
	// Storage: Task Tasks (r:1 w:1)
	
	// Storage: Task TasksOwned (r:1 w:1)
	
	fn reject_task(_s: u32, _x: u32, ) -> Weight {
		(23_789_000 as Weight)
			
			
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			
			
			
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
			
			
	}
	
}
