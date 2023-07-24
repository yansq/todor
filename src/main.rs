use anyhow::anyhow;
use std::path::PathBuf;
use structopt::StructOpt;

mod cli;
mod tasks;

use cli::{Action::*, CommandLineArgs};
use tasks::Task;

fn find_default_output_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-todo.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    let CommandLineArgs {
        action,
        output_file,
    } = CommandLineArgs::from_args();

    let output_file = output_file
        .or_else(find_default_output_file)
        .ok_or(anyhow!("Failed to find output file"))?;

    match action {
        Add { text } => tasks::add_task(output_file, Task::new(text)),
        List => tasks::list_tasks(output_file),
        Done { position } => tasks::complete_task(output_file, position),
    }?;

    Ok(())
}
