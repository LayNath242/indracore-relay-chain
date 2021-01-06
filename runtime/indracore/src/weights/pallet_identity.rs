// Copyright 2017-2020 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for pallet_identity
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 2.0.0
//! DATE: 2020-12-08, STEPS: [50, ], REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("indracore-dev"), DB CACHE: 128

// Executed Command:
// target/release/indracore
// benchmark
// --chain=indracore-dev
// --steps=50
// --repeat=20
// --pallet=pallet_identity
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./file_header.txt
// --output=./runtime/indracore/src/weights/


#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for pallet_identity.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_identity::WeightInfo for WeightInfo<T> {
	fn add_registrar(r: u32, ) -> Weight {
		(28_261_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((318_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_identity(r: u32, x: u32, ) -> Weight {
		(73_360_000 as Weight)
			// Standard Error: 19_000
			.saturating_add((234_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 2_000
			.saturating_add((1_863_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_subs_new(s: u32, ) -> Weight {
		(52_544_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((9_959_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	fn set_subs_old(p: u32, ) -> Weight {
		(48_351_000 as Weight)
			// Standard Error: 0
			.saturating_add((3_391_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(p as Weight)))
	}
	fn clear_identity(r: u32, s: u32, x: u32, ) -> Weight {
		(62_001_000 as Weight)
			// Standard Error: 8_000
			.saturating_add((171_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 0
			.saturating_add((3_390_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: 0
			.saturating_add((1_089_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	fn request_judgement(r: u32, x: u32, ) -> Weight {
		(74_257_000 as Weight)
			// Standard Error: 8_000
			.saturating_add((334_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 1_000
			.saturating_add((2_141_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn cancel_request(r: u32, x: u32, ) -> Weight {
		(62_893_000 as Weight)
			// Standard Error: 11_000
			.saturating_add((231_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 1_000
			.saturating_add((2_117_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_fee(r: u32, ) -> Weight {
		(10_890_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((268_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_account_id(r: u32, ) -> Weight {
		(12_410_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((268_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_fields(r: u32, ) -> Weight {
		(10_855_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((269_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn provide_judgement(r: u32, x: u32, ) -> Weight {
		(49_519_000 as Weight)
			// Standard Error: 9_000
			.saturating_add((299_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 1_000
			.saturating_add((2_127_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn kill_identity(r: u32, s: u32, x: u32, ) -> Weight {
		(103_419_000 as Weight)
			// Standard Error: 5_000
			.saturating_add((120_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 0
			.saturating_add((3_400_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: 0
			.saturating_add((3_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	fn add_sub(s: u32, ) -> Weight {
		(72_490_000 as Weight)
			// Standard Error: 0
			.saturating_add((191_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn rename_sub(s: u32, ) -> Weight {
		(23_454_000 as Weight)
			// Standard Error: 0
			.saturating_add((25_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn remove_sub(s: u32, ) -> Weight {
		(69_012_000 as Weight)
			// Standard Error: 0
			.saturating_add((164_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn quit_sub(s: u32, ) -> Weight {
		(45_725_000 as Weight)
			// Standard Error: 0
			.saturating_add((158_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}
