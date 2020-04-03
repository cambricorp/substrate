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
use crate::params::ImportParams;
use crate::params::SharedParams;
use crate::CliConfiguration;
//use sc_service::{Configuration, ServiceBuilderCommand};
use sp_runtime::traits::{Block as BlockT, Header as HeaderT};
use std::fmt::Debug;
use std::fs;
use std::io::{self, Read, Seek};
use std::path::PathBuf;
use structopt::StructOpt;
use sc_service::import_blocks;

/// The `import-blocks` command used to import blocks.
#[derive(Debug, StructOpt, Clone)]
pub struct ImportBlocksCmd {
	/// Input file or stdin if unspecified.
	// NOTE: this is an option so implementations can set their own defaults
	#[structopt(parse(from_os_str))]
	pub input: Option<PathBuf>,

	/// The default number of 64KB pages to ever allocate for Wasm execution.
	///
	/// Don't alter this unless you know what you're doing.
	// NOTE: this is an option so implementations can set their own defaults
	#[structopt(long = "default-heap-pages", value_name = "COUNT")]
	pub default_heap_pages: Option<u32>,

	#[allow(missing_docs)]
	#[structopt(flatten)]
	pub shared_params: SharedParams,

	#[allow(missing_docs)]
	#[structopt(flatten)]
	pub import_params: ImportParams,
}

/// Internal trait used to cast to a dynamic type that implements Read and Seek.
trait ReadPlusSeek: Read + Seek {}

impl<T: Read + Seek> ReadPlusSeek for T {}

impl ImportBlocksCmd {
	/// Run the import-blocks command
	pub async fn run<B, BA, CE, IQ>(
		&self,
		client: std::sync::Arc<sc_service::Client<BA, CE, B, ()>>,
		import_queue: IQ,
	) -> error::Result<()>
	where
		B: BlockT,
		BA: sc_client_api::backend::Backend<B> + 'static,
		CE: sc_client_api::call_executor::CallExecutor<B> + Send + Sync + 'static,
		IQ: sc_service::ImportQueue<B> + Sync + 'static,
	{
		let file: Box<dyn ReadPlusSeek + Send> = match &self.input {
			Some(filename) => Box::new(fs::File::open(filename)?),
			None => {
				let mut buffer = Vec::new();
				io::stdin().read_to_end(&mut buffer)?;
				Box::new(io::Cursor::new(buffer))
			}
		};

		import_blocks(client, import_queue, file, false)
			.await
			.map_err(|e| e.into())
	}
}

impl CliConfiguration for ImportBlocksCmd {
	fn shared_params(&self) -> &SharedParams {
		&self.shared_params
	}

	fn import_params(&self) -> Option<&ImportParams> {
		Some(&self.import_params)
	}
}
