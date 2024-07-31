//Local Imports
mod args;

//Function Imports
use args::{CatArgs, CliTool, EntityType};
use clap::Parser;
use std::fs::{self, DirEntry, File};
use std::io::{Read, Result};

fn main() {
    let matches = CliTool::parse();
    match matches.entity_type {
        EntityType::Echo(some_string) => println!("{:?}", some_string.repeated),
        EntityType::List(some_argument) => match list_function(
            &some_argument.directory,
            some_argument.all,
            some_argument.long,
        ) {
            Ok(_) => (),
            Err(e) => eprintln!("Error : {:?}", e),
        },
        EntityType::Cat(cat_argument) => match cat_argument {
            CatArgs::New(new_args) => {
                match concatenate_new_function(new_args.new_concat_file, &new_args.files) {
                    Ok(_) => (),
                    Err(e) => eprintln!("Error : {:?}", e),
                }
            }
            CatArgs::Same(same_args) => match concatenate_same_function(&same_args.files) {
                Ok(_) => (),
                Err(e) => eprintln!("Error : {:?}", e),
            },
            CatArgs::Dir(dir_args) => match concatenate_dir_function(&dir_args.directory) {
                Ok(_) => (),
                Err(e) => eprintln!("Error : {:?}", e),
            },
        },
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

fn concatenate_new_function(concat_file: String, files: &Option<Vec<String>>) -> Result<()> {
    let mut contents = String::new();
    match files {
        Some(some_file) => {
            for file in some_file {
                let mut f = File::open(file)?;
                let mut temp_content = String::new();
                f.read_to_string(&mut temp_content)?;
                contents.push_str(&temp_content);
            }
            fs::write(&concat_file, contents)?;
        }
        None => println!("No files are available"),
    }
    Ok(())
}

fn concatenate_same_function(files: &Option<Vec<String>>) -> Result<()> {
    let mut contents = String::new();
    match files {
        Some(some_file) => {
            for file in some_file {
                let mut f = File::open(file)?;
                let mut temp_content = String::new();
                f.read_to_string(&mut temp_content)?;
                contents.push_str(&temp_content);
            }
            fs::write(some_file[0].clone(), contents)?;
        }
        None => println!("No files are available"),
    }
    Ok(())
}

fn concatenate_dir_function(directory: &str) -> Result<()> {
    let mut contents = String::new();
    let paths = fs::read_dir(directory)?;
    for path in paths {
        let entry = path?;
        let mut f = File::open(entry.file_name())?;
        let mut temp_content = String::new();
        f.read_to_string(&mut temp_content)?;
        contents.push_str(&temp_content);
    }
    fs::write("dir_summation.txt", &contents)?;
    Ok(())
}
