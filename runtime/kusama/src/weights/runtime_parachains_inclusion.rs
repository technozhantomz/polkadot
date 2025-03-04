// Copyright 2017-2022 Parity Technologies (UK) Ltd.
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
//! Autogenerated weights for `runtime_parachains::inclusion`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-23, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `i9`, CPU: `13th Gen Intel(R) Core(TM) i9-13900K`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kusama-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/polkadot
// benchmark
// pallet
// --chain=kusama-dev
// --steps=50
// --repeat=20
// --pallet=runtime_parachains::inclusion
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/kusama/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `runtime_parachains::inclusion`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_parachains::inclusion::WeightInfo for WeightInfo<T> {
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:1 w:999)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65585), added: 68060, mode: MaxEncodedLen)
	/// The range of component `i` is `[1, 1000]`.
	fn receive_upward_messages(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `51490`
		//  Estimated: `70587`
		// Minimum execution time: 47_556 nanoseconds.
		Weight::from_parts(48_839_000, 0)
			.saturating_add(Weight::from_parts(0, 70587))
			// Standard Error: 49_019
			.saturating_add(Weight::from_parts(43_136_059, 0).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i.into())))
	}
}
