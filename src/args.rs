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

#[derive(Debug, Args)]
pub struct CatArgs {
    ///File 1
    #[arg(default_value = "./foo.txt")]
    pub file_name: String,
    /// File 2
    #[arg(default_value = "./foo2.txt")]
    pub another_file: String,
}
