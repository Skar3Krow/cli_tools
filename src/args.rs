use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version)]
pub struct CliTool {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Repeats User Input
    Echo(Repeat),
    /// Lists all the files and directories
    List(ListArgs),
    /// Concatenates files
    #[clap(subcommand)]
    Cat(CatArgs),
}

#[derive(Debug, Args)]
pub struct Repeat {
    /// The sentence to be repeated
    #[arg(default_value = "*cold silence of the universe*")]
    pub repeated: String,
}

#[derive(Debug, Args)]
pub struct ListArgs {
    /// Lists the directory
    #[arg(default_value = ".")]
    pub directory: String,
    /// Lists all the hidden files as
    #[arg(long, short)]
    pub all: bool,
    /// Lists in a long listing format
    #[arg(long, short)]
    pub long: bool,
}

#[derive(Debug, Parser)]
pub enum CatArgs {
    /// Creates a new concatenated file
    New(NewArgs),
    /// Concatenates a directory
    Dir(DirArgs),
    /// Concatenates in the same directory
    Same(SameArgs),
}

#[derive(Debug, Args)]
pub struct NewArgs {
    /// New file to be concatenated
    #[arg(default_value = "concat_file")]
    pub new_concat_file: String,
    /// Takes n number of files as input
    pub files: Option<Vec<String>>,
}

#[derive(Debug, Args)]
pub struct SameArgs {
    /// Takes n number of files as input
    pub files: Option<Vec<String>>,
}

#[derive(Debug, Args)]
pub struct DirArgs {
    /// Takes a directory as input
    #[arg(default_value = ".")]
    pub directory: String,
}

// #[derive(Debug, Args)]
// pub struct CatArgs {
//     /// Creates a new concatenated file
//     #[arg(long, short)]
//     pub new: bool,
//     /// Takes n number of files as input
//     pub files: Option<Vec<String>>,
// }
