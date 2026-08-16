mod cli;
mod helpers;
mod code_templates;

use std::io::Result;
use cli::NXCli;
use structopt::StructOpt;

use crate::cli::NXCommand;

fn main() -> Result<()> {
    let args: NXCli = NXCli::from_args();

    match args.cmd {
        NXCommand::New(args) => helpers::new(&args)
    }
}
