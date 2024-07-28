//Local Imports
mod args;

//Function Imports
use args::{CliTool, EntityType};
use clap::Parser;
use std::fs::{self, DirEntry};
use std::io::Result;

fn main() {
    let matches = CliTool::parse();
    match matches.entity_type {
        EntityType::Echo(some_string) => println!("{:?}", some_string.repeated),
        EntityType::List(some_argument) => {
            match list_function(
                &some_argument.directory,
                some_argument.all,
                some_argument.long,
            ) {
                Ok(_) => (),
                Err(e) => eprintln!("Error : {:?}", e),
            }
        }
    };
}

fn list_function(directory: &str, display_all: bool, long_format: bool) -> Result<()> {
    match fs::read_dir(directory) {
        Ok(paths) => {
            for path in paths {
                let entry = path?;
                if !display_all && hidden_file(&entry) {
                    continue;
                }
                if long_format {
                    match print_long_format(&entry) {
                        Ok(_) => (),
                        Err(e) => println!("Error Occured: {:?}", e),
                    };
                } else {
                    println!("{:?}", entry.file_name().to_string_lossy());
                }
            }
        }
        Err(e) => eprintln!("Error Occured {:?}", e),
    };
    Ok(())
}

fn hidden_file(entry: &DirEntry) -> bool {
    entry.file_name().to_string_lossy().starts_with('.')
}

fn print_long_format(entry: &DirEntry) -> Result<()> {
    let file_metadata = entry.metadata()?;
    let file_type = if file_metadata.is_dir() { "d" } else { "f" };
    let file_size = file_metadata.len();
    println!(
        "{:<3} {:<5} {:?}",
        file_type,
        file_size,
        entry.file_name().to_string_lossy()
    );
    Ok(())
}
