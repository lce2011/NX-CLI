use structopt::StructOpt;

#[derive(StructOpt)]
pub struct NXCli {
    #[structopt(subcommand)]
    pub cmd: NXCommand
}

#[derive(StructOpt)]
pub enum NXCommand {
    #[structopt(about="Create a new project", name="new")]
    New(NewOptions),
    #[structopt(about="Update your enviroment", name="update")]
    Update
}

#[derive(StructOpt)]
pub struct NewOptions {
    #[structopt(long, short)]
    pub name: String,
    #[structopt(long, short)]
    pub lang: Option<String>,
    #[structopt(long, short)]
    pub empty: Option<bool>
}
