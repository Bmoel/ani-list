use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug,StructOpt)]
pub enum Action {
    /// Add an anime to the list with args
    Add,
    /// Update an existing anime in the list
    Update,
    /// Remove an existing anime in the list
    Remove,
    /// Search and display an anime in the list
    Search,
    /// Export current anime list to a separate text file
    Export,
    /// List all anime to the command line
    List,
}

#[derive(Debug,StructOpt)]
#[structopt(
    name = "Anime List",
    about = "\nA way to store your favorite anime\nCreated by Bmoe, based on list cli app in Microsoft's Rust course:\nhttps://docs.microsoft.com/en-us/learn/modules/rust-create-command-line-program/",
)]

pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,
    #[structopt(parse(from_os_str),short,long)]
    /// Will write to a file in your home directory if not specified
    pub file_name: Option<PathBuf>,
}