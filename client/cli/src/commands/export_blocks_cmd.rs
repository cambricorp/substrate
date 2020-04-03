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
use log::info;
use sc_service::{
	config::DatabaseConfig, //Configuration, ServiceBuilderCommand,
};
use sp_runtime::traits::{Block as BlockT, Header as HeaderT};
use std::fmt::Debug;
use std::fs;
use std::io;
use std::path::PathBuf;
use structopt::StructOpt;
use sc_service::export_blocks;

/// The `export-blocks` command used to export blocks.
#[derive(Debug, StructOpt, Clone)]
pub struct ExportBlocksCmd {
	/// Output file name or stdout if unspecified.
	// NOTE: this is an option so implementations can set their own defaults
	#[structopt(parse(from_os_str))]
	pub output: Option<PathBuf>,

	/// Specify starting block number.
	///
	/// Default is 1.
	// NOTE: this is an option so implementations can set their own defaults
	#[structopt(long = "from", value_name = "BLOCK")]
	pub from: Option<BlockNumber>,

	/// Specify last block number.
	///
	/// Default is best block.
	// NOTE: this is an option so implementations can set their own defaults
	#[structopt(long = "to", value_name = "BLOCK")]
	pub to: Option<BlockNumber>,

	/// Use binary output rather than JSON.
	#[structopt(long = "binary", value_name = "BOOL", parse(try_from_str), default_value("false"))]
	pub binary: bool,

	#[allow(missing_docs)]
	#[structopt(flatten)]
	pub shared_params: SharedParams,

	#[allow(missing_docs)]
	#[structopt(flatten)]
	pub pruning_params: PruningParams,
}

impl ExportBlocksCmd {
	/// Run the export-blocks command
	pub async fn run<B, BA, CE>(
		&self,
		client: std::sync::Arc<sc_service::Client<BA, CE, B, ()>>,
	) -> error::Result<()>
	where
		B: BlockT,
		BA: sc_client_api::backend::Backend<B> + 'static,
		CE: sc_client_api::call_executor::CallExecutor<B> + Send + Sync + 'static,
		<<<B as sp_runtime::traits::Block>::Header as sp_runtime::traits::Header>::Number as std::str::FromStr>::Err: std::fmt::Debug,
	{
		// TODO: should probably not be here
		/*
		if let DatabaseConfig::Path { ref path, .. } = &config.database {
			info!("DB path: {}", path.display());
		}
		*/

		let from = self.from.as_ref().and_then(|f| f.parse().ok()).unwrap_or(1);
		let to = self.to.as_ref().and_then(|t| t.parse().ok());

		let binary = self.binary;

		let file: Box<dyn io::Write> = match &self.output {
			Some(filename) => Box::new(fs::File::create(filename)?),
			None => Box::new(io::stdout()),
		};

		export_blocks(client, file, from.into(), to, binary)
			.await
			.map_err(|e| e.into())
	}
}

impl CliConfiguration for ExportBlocksCmd {
	fn shared_params(&self) -> &SharedParams {
		&self.shared_params
	}

	fn pruning_params(&self) -> Option<&PruningParams> {
		Some(&self.pruning_params)
	}
}
