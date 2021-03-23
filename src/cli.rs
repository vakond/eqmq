//! eqmq options parser.

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Command {
    #[structopt(about = "Starts processing")]
    Run,
}

#[derive(StructOpt)]
#[structopt(about = "Equilibrium MQ processor")]
pub struct Application {
    #[structopt(
        parse(from_os_str),
        short,
        long,
        help = "Path to the config file (default: eqmq.json)"
    )]
    pub config: Option<PathBuf>,

    #[structopt(subcommand)]
    pub cmd: Command,
}

/// Constructs instance of Application.
pub fn application() -> Application {
    Application::from_args()
}
