use std::io::{Error, ErrorKind, Result, Write};
use std::fs::{File, create_dir};
use std::env::current_dir;
use std::path::Path;

use crate::cli::NewOptions;
use crate::code_templates::{C_CXX_CODE, MAKEFILE_CODE};

pub fn new(args: &NewOptions) -> Result<()> {
    let project_path: String = build_path_from_cwd(&args.name.to_string());

    if Path::new(&project_path).exists() {
        Err(Error::new(ErrorKind::AlreadyExists, "A folder with this name already exists!"))
    } else {
        let _ = create_dir(&project_path);
        let source_path = build_path_from_two(&project_path, "source");
        let _ = create_dir(&source_path);

        let main_code_path: String;

        if &args.lang.clone().unwrap().to_string().to_lowercase() == "c" {
            main_code_path = build_path_from_two(&source_path, "main.c");
        } else if &args.lang.clone().unwrap().to_string().to_lowercase() == "cpp" || &args.lang.clone().unwrap().to_string().to_lowercase() == "cxx" {
            main_code_path = build_path_from_two(&source_path, "main.cpp");
        } else {
            return Err(Error::new(ErrorKind::InvalidFilename, "Unknown source code type! Consider using C, CPP or CXX."));
        }
        let makefile_path = build_path_from_two(&project_path, "Makefile");

        let mut main_code_file = File::create(main_code_path)?;
        let mut makefile_code_file = File::create(makefile_path)?;

        if args.empty.clone().unwrap() {
            main_code_file.write_all("".as_bytes())?;
            makefile_code_file.write_all("".as_bytes())?;
        }else {
            main_code_file.write_all(C_CXX_CODE.as_bytes())?;
            makefile_code_file.write_all(MAKEFILE_CODE.as_bytes())?;
        }

        Ok(())
    }
}

fn build_path_from_cwd(name: &str) -> String {
    let cwd = current_dir();
    return cwd.unwrap().to_string_lossy().to_string() + "/" + name;
}

fn build_path_from_two(first: &str, second: &str) -> String {
    return first.to_string() + "/" + second;
}
