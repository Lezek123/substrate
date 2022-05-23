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

//! Autogenerated weights for pallet_recovery
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-05-22, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/substrate
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_recovery
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --template=./.maintain/frame-weight-template.hbs
// --output=./frame/recovery/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{ComputationWeight as Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_recovery.
pub trait WeightInfo {
	fn as_recovered() -> Weight;
	fn set_recovered() -> Weight;
	fn create_recovery(n: u32, ) -> Weight;
	fn initiate_recovery() -> Weight;
	fn vouch_recovery(n: u32, ) -> Weight;
	fn claim_recovery(n: u32, ) -> Weight;
	fn close_recovery(n: u32, ) -> Weight;
	fn remove_recovery(n: u32, ) -> Weight;
	fn cancel_recovered() -> Weight;
}

/// Weights for pallet_recovery using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Recovery Proxy (r:1 w:0)
	fn as_recovered() -> Weight {
		(3_732_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	// Storage: Recovery Proxy (r:0 w:1)
	fn set_recovered() -> Weight {
		(10_201_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Recovery Recoverable (r:1 w:1)
	fn create_recovery(n: u32, ) -> Weight {
		(23_334_000 as Weight)
			// Standard Error: 13_000
			.saturating_add((201_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Recovery Recoverable (r:1 w:0)
	// Storage: Recovery ActiveRecoveries (r:1 w:1)
	fn initiate_recovery() -> Weight {
		(28_358_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Recovery Recoverable (r:1 w:0)
	// Storage: Recovery ActiveRecoveries (r:1 w:1)
	fn vouch_recovery(n: u32, ) -> Weight {
		(17_667_000 as Weight)
			// Standard Error: 13_000
			.saturating_add((342_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Recovery Recoverable (r:1 w:0)
	// Storage: Recovery ActiveRecoveries (r:1 w:0)
	// Storage: Recovery Proxy (r:1 w:1)
	fn claim_recovery(n: u32, ) -> Weight {
		(24_591_000 as Weight)
			// Standard Error: 14_000
			.saturating_add((242_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Recovery ActiveRecoveries (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn close_recovery(n: u32, ) -> Weight {
		(28_763_000 as Weight)
			// Standard Error: 12_000
			.saturating_add((142_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Recovery ActiveRecoveries (r:1 w:0)
	// Storage: Recovery Recoverable (r:1 w:1)
	fn remove_recovery(n: u32, ) -> Weight {
		(27_357_000 as Weight)
			// Standard Error: 17_000
			.saturating_add((212_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Recovery Proxy (r:1 w:1)
	fn cancel_recovered() -> Weight {
		(9_068_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Recovery Proxy (r:1 w:0)
	fn as_recovered() -> Weight {
		(3_732_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
	}
	// Storage: Recovery Proxy (r:0 w:1)
	fn set_recovered() -> Weight {
		(10_201_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Recovery Recoverable (r:1 w:1)
	fn create_recovery(n: u32, ) -> Weight {
		(23_334_000 as Weight)
			// Standard Error: 13_000
			.saturating_add((201_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Recovery Recoverable (r:1 w:0)
	// Storage: Recovery ActiveRecoveries (r:1 w:1)
	fn initiate_recovery() -> Weight {
		(28_358_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Recovery Recoverable (r:1 w:0)
	// Storage: Recovery ActiveRecoveries (r:1 w:1)
	fn vouch_recovery(n: u32, ) -> Weight {
		(17_667_000 as Weight)
			// Standard Error: 13_000
			.saturating_add((342_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Recovery Recoverable (r:1 w:0)
	// Storage: Recovery ActiveRecoveries (r:1 w:0)
	// Storage: Recovery Proxy (r:1 w:1)
	fn claim_recovery(n: u32, ) -> Weight {
		(24_591_000 as Weight)
			// Standard Error: 14_000
			.saturating_add((242_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Recovery ActiveRecoveries (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn close_recovery(n: u32, ) -> Weight {
		(28_763_000 as Weight)
			// Standard Error: 12_000
			.saturating_add((142_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Recovery ActiveRecoveries (r:1 w:0)
	// Storage: Recovery Recoverable (r:1 w:1)
	fn remove_recovery(n: u32, ) -> Weight {
		(27_357_000 as Weight)
			// Standard Error: 17_000
			.saturating_add((212_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Recovery Proxy (r:1 w:1)
	fn cancel_recovered() -> Weight {
		(9_068_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}
