mod cli;
mod helpers;
mod code_templates;
mod handlers;
mod error;

use std::error::Error;

use structopt::StructOpt;

use cli::NXCli;
use crate::cli::NXCommand;

fn main() -> Result<(), Box<dyn Error>> {
    let args: NXCli = NXCli::from_args_safe()?;

    match args.cmd {
        NXCommand::New(args) => Ok(helpers::new(&args)?),
        NXCommand::Update => helpers::update()
    }
}
