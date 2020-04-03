// Copyright 2018-2020 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

use crate::error;
use crate::params::{BlockNumber, PruningParams, SharedParams};
use crate::CliConfiguration;
//use sc_service::{Configuration, ServiceBuilderCommand};
use sp_runtime::traits::{Block as BlockT, Header as HeaderT};
use std::fmt::Debug;
use structopt::StructOpt;
use sc_service::revert_chain;

/// The `revert` command used revert the chain to a previous state.
#[derive(Debug, StructOpt, Clone)]
pub struct RevertCmd {
	/// Number of blocks to revert.
	#[structopt(default_value = "256")]
	pub num: BlockNumber,

	#[allow(missing_docs)]
	#[structopt(flatten)]
	pub shared_params: SharedParams,

	#[allow(missing_docs)]
	#[structopt(flatten)]
	pub pruning_params: PruningParams,
}

impl RevertCmd {
	/// Run the revert command
	pub fn run<B, BA, CE>(&self, client: std::sync::Arc<sc_service::Client<BA, CE, B, ()>>) -> error::Result<()>
	where
		B: BlockT,
		BA: sc_client_api::backend::Backend<B> + 'static,
		CE: sc_client_api::call_executor::CallExecutor<B> + Send + Sync + 'static,
		<<<B as sp_runtime::traits::Block>::Header as sp_runtime::traits::Header>::Number as std::str::FromStr>::Err: std::fmt::Debug,
	{
		let blocks = self.num.parse()?;
		revert_chain(client, blocks)?;

		Ok(())
	}
}

impl CliConfiguration for RevertCmd {
	fn shared_params(&self) -> &SharedParams {
		&self.shared_params
	}

	fn pruning_params(&self) -> Option<&PruningParams> {
		Some(&self.pruning_params)
	}
}
