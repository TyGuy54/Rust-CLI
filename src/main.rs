#![allow(unused)]
use clap::Parser;
use std::path::Path;
use std::fs::File;
use std::io::Result;
use std::env;
use std::fs;

#[derive(Parser)]
#[clap(author = "Tyler Ross", version, about)]
/// make a directory with a file in it 
struct CliArgs {
    dir_name: String,
    file_name: String,
    location: String
}

fn main() -> Result<()>{
    let args = CliArgs::parse();

    let root = Path::new(&args.location);
    assert!(env::set_current_dir(&root).is_ok());
    
    fs::create_dir(&args.dir_name);

    if Path::new(&args.dir_name).exists() {
        let root = Path::new(&args.dir_name);
        assert!(env::set_current_dir(&root).is_ok());

        File::create(&args.file_name);
    }

    return Ok(())
}

