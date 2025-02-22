// This file is part of Substrate.

// Copyright (C) 2021 Parity Technologies (UK) Ltd.
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

//! Autogenerated weights for pallet_tips
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-03-15, STEPS: 50, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/deeper-chain
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_tips
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./pallets/tips/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_tips.
pub trait WeightInfo {
    fn report_awesome(r: u32) -> Weight;
    fn retract_tip() -> Weight;
    fn tip_new(r: u32, t: u32) -> Weight;
    fn credit_tip_new(r: u32, t: u32) -> Weight;
    fn tip(t: u32) -> Weight;
    fn credit_tip(t: u32) -> Weight;
    fn close_tip(t: u32) -> Weight;
    fn close_credit_tip(t: u32) -> Weight;
    fn slash_tip(t: u32) -> Weight;
}

/// Weights for pallet_tips using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn report_awesome(r: u32) -> Weight {
        (30_376_000 as Weight) // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(r as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    fn retract_tip() -> Weight {
        (28_891_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    fn tip_new(r: u32, t: u32) -> Weight {
        (19_883_000 as Weight) // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(r as Weight)) // Standard Error: 0
            .saturating_add((142_000 as Weight).saturating_mul(t as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    fn credit_tip_new(r: u32, t: u32) -> Weight {
        (19_462_000 as Weight) // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(r as Weight)) // Standard Error: 0
            .saturating_add((141_000 as Weight).saturating_mul(t as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    fn tip(t: u32) -> Weight {
        (13_421_000 as Weight) // Standard Error: 0
            .saturating_add((581_000 as Weight).saturating_mul(t as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn credit_tip(t: u32) -> Weight {
        (13_042_000 as Weight) // Standard Error: 0
            .saturating_add((566_000 as Weight).saturating_mul(t as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn close_tip(t: u32) -> Weight {
        (51_394_000 as Weight) // Standard Error: 1_000
            .saturating_add((360_000 as Weight).saturating_mul(t as Weight))
            .saturating_add(T::DbWeight::get().reads(5 as Weight))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
    }
    fn close_credit_tip(t: u32) -> Weight {
        (27_990_000 as Weight) // Standard Error: 0
            .saturating_add((366_000 as Weight).saturating_mul(t as Weight))
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    fn slash_tip(t: u32) -> Weight {
        (17_164_000 as Weight) // Standard Error: 0
            .saturating_add((13_000 as Weight).saturating_mul(t as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn report_awesome(r: u32) -> Weight {
        (30_376_000 as Weight) // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(r as Weight))
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
    }
    fn retract_tip() -> Weight {
        (28_891_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
    }
    fn tip_new(r: u32, t: u32) -> Weight {
        (19_883_000 as Weight) // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(r as Weight)) // Standard Error: 0
            .saturating_add((142_000 as Weight).saturating_mul(t as Weight))
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
    }
    fn credit_tip_new(r: u32, t: u32) -> Weight {
        (19_462_000 as Weight) // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(r as Weight)) // Standard Error: 0
            .saturating_add((141_000 as Weight).saturating_mul(t as Weight))
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
    }
    fn tip(t: u32) -> Weight {
        (13_421_000 as Weight) // Standard Error: 0
            .saturating_add((581_000 as Weight).saturating_mul(t as Weight))
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    fn credit_tip(t: u32) -> Weight {
        (13_042_000 as Weight) // Standard Error: 0
            .saturating_add((566_000 as Weight).saturating_mul(t as Weight))
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    fn close_tip(t: u32) -> Weight {
        (51_394_000 as Weight) // Standard Error: 1_000
            .saturating_add((360_000 as Weight).saturating_mul(t as Weight))
            .saturating_add(RocksDbWeight::get().reads(5 as Weight))
            .saturating_add(RocksDbWeight::get().writes(4 as Weight))
    }
    fn close_credit_tip(t: u32) -> Weight {
        (27_990_000 as Weight) // Standard Error: 0
            .saturating_add((366_000 as Weight).saturating_mul(t as Weight))
            .saturating_add(RocksDbWeight::get().reads(3 as Weight))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
    }
    fn slash_tip(t: u32) -> Weight {
        (17_164_000 as Weight) // Standard Error: 0
            .saturating_add((13_000 as Weight).saturating_mul(t as Weight))
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
    }
}
