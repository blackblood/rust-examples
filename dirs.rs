use std::fs;
use std::io;
use std::env;
use std::path::PathBuf;

fn explore(file_path: &PathBuf) -> Result<(), io::Error> {
	let meta_data = fs::metadata(file_path)?;
	let file_type = meta_data.file_type();
	
	if file_type.is_dir() {
		let entries = fs::read_dir(file_path)?;
		for entry in entries {
			explore(&entry?.path());
		}
	} else {
		println!("file name: {:?}", file_path);
		if let Some(ext) = file_path.extension() {
			if let Some(s) = ext.to_str() {
				println!("extension: {}", s);
			}
		}
	}
	return Ok(());
}

fn main() {
	let current_directory = env::current_dir().expect("invalid path");
	if let Err(error) = explore(&current_directory) {
		println!("{}", error);
	}
}