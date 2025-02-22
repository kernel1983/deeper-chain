// This file is part of Substrate.

// Copyright (C) 2018-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use clap::Parser;
use sc_cli::{SignCmd, VanityCmd, VerifyCmd};

#[allow(missing_docs)]
#[derive(Debug, Parser)]
pub struct RunCmd {
    #[allow(missing_docs)]
    #[clap(flatten)]
    pub base: sc_cli::RunCmd,

    #[clap(long)]
    pub enable_dev_signer: bool,

    /// Maximum number of logs in a query.
    #[clap(long, default_value = "10000")]
    pub max_past_logs: u32,

    /// Maximum fee history cache size.
    #[clap(long, default_value = "2048")]
    pub fee_history_limit: u64,

    /// The dynamic-fee pallet target gas price set by block author
    #[clap(long, default_value = "1")]
    pub target_gas_price: u64,
}

/// An overarching CLI command definition.
#[derive(Debug, clap::Parser)]
pub struct Cli {
    /// Possible subcommand with parameters.
    #[clap(subcommand)]
    pub subcommand: Option<Subcommand>,
    #[allow(missing_docs)]
    #[clap(flatten)]
    pub run: RunCmd,
}

/// Possible subcommands of the main binary.
#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
    /// Key management cli utilities
    #[clap(subcommand)]
    Key(sc_cli::KeySubcommand),

    /// The custom inspect subcommmand for decoding blocks and extrinsics.
    #[clap(
        name = "inspect",
        about = "Decode given block or extrinsic using current native runtime."
    )]
    Inspect(node_inspect::cli::InspectCmd),

    /// The custom benchmark subcommmand benchmarking runtime pallets.
    #[clap(name = "benchmark", about = "Benchmark runtime pallets.")]
    Benchmark(frame_benchmarking_cli::BenchmarkCmd),

    /// Verify a signature for a message, provided on STDIN, with a given (public or secret) key.
    Verify(VerifyCmd),

    /// Generate a seed that provides a vanity address.
    Vanity(VanityCmd),

    /// Sign a message, with a given (secret) key.
    Sign(SignCmd),

    /// Build a chain specification.
    BuildSpec(sc_cli::BuildSpecCmd),

    /// Validate blocks.
    CheckBlock(sc_cli::CheckBlockCmd),

    /// Export blocks.
    ExportBlocks(sc_cli::ExportBlocksCmd),

    /// Export the state of a given block into a chain spec.
    ExportState(sc_cli::ExportStateCmd),

    /// Import blocks.
    ImportBlocks(sc_cli::ImportBlocksCmd),

    /// Remove the whole chain.
    PurgeChain(sc_cli::PurgeChainCmd),

    /// Revert the chain to a previous state.
    Revert(sc_cli::RevertCmd),
}
