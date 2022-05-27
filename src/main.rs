use anyhow::anyhow;
use std::path::PathBuf;
use structopt::StructOpt;

mod cli;
use cli::{Action::*, CommandLineArgs};

mod tasks;

fn find_default_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push("anilist.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    let CommandLineArgs {
        action,
        file_name,
    } = CommandLineArgs::from_args();

    let file_name = file_name
        .or_else(find_default_file)
        .ok_or(anyhow!("file name not found"))?;
    
    match action {
        Add => tasks::add_anime(file_name),
        Update => tasks::update_anime(file_name),
        Remove => tasks::remove_anime(file_name),
        Search => tasks::search_anime(file_name),
        Export => tasks::export_anime(file_name),
        List => tasks::list_anime(file_name),
    }?;
    Ok(())
}