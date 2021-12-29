use chrono::{DateTime, Local};
use std::error::Error;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

#[macro_use]
#[derive(StructOpt, Debug)]
struct Opt {
    /// Output file
    #[structopt(default_value = ".", parse(from_os_str))]
    path: PathBuf,
}

fn main() {
    let opt = Opt::from_args();

    if let Err(ref e) = run(&opt.path) {
        println!("{}", e);
        process::exit(1);
    }
}

fn run(dir: &PathBuf) -> Result<(), Box<dyn Error>> {
    if !dir.is_dir() {
        println!("Directory not found");
        process::exit(1);
    } else {
        for entry in fs::read_dir(dir)? {
            ls(entry?);
        }
    }

    Ok(())
}

fn ls(entry: DirEntry) {
    let file_name = entry
        .file_name()
        .into_string()
        .or_else(|f| Err(format!("Invalid entry: {:?}", f)))
        .unwrap();

    let metadata = entry.metadata().unwrap();
    let size = metadata.len();
    let modified: DateTime<Local> = DateTime::from(metadata.modified().unwrap());

    println!(
        "{:>5} {} {}",
        size,
        modified.format("%_d %b %H:%M").to_string(),
        file_name
    );
}
