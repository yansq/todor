use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Write tasks to the file.
    Add {
        #[structopt()]
        text: String,
    },
    /// Remove an entry from the file by position.
    Done {
        #[structopt()]
        position: usize,
    },
    /// List all takss in the file.
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Rusty Todo",
    about = "A command line to-do app written in Rust"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,
    /// Use a different output file.
    #[structopt(parse(from_os_str), short, long)]
    pub output_file: Option<PathBuf>,
}

