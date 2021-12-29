use std::error::Error;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use std::process;

fn main() {
    if let Err(ref e) = run(Path::new(".")) {
        println!("{}", e);
        process::exit(1);
    }
}

fn run(dir: &Path) -> Result<(), Box<dyn Error>> {
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
    let path = entry.path();
    let file_name = entry
        .file_name()
        .into_string()
        .or_else(|f| Err(format!("Invalid entry: {:?}", f)))
        .unwrap();

    let mut output_string = String::from("");
    if path.is_dir() {
        output_string.push_str("/");
    }
    output_string.push_str(&file_name);
    println!("{}", output_string);
}
