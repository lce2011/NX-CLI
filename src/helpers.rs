use std::io::{Error, ErrorKind, Result, Write};
use std::fs::{File, create_dir, remove_file};
use std::path::Path;
use std::env::current_dir;
use std::process::Command;

use crate::cli::NewOptions;
use crate::code_templates::{C_CXX_CODE, MAKEFILE_CODE};
use crate::error::{CustomError, CustomErrorKind};
use crate::handlers::{build_path_from_cwd, build_path_from_two, get_os, download_latest_devkit_release_windows, execute_updater_windows};
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

pub fn update() -> core::result::Result<(), Box<dyn std::error::Error>> {
    println!("Checking for wget...");
    let wget_avaible: bool = Command::new("wget")
        .output()
        .is_ok();

    if !wget_avaible {
        Err(CustomError::new(CustomErrorKind::WgetUnavaible, "Couldn't find the command wget in your PATH. Is wget installed?"))?;
    }
    println!("Found wget in PATH.");

    let os_binding: String = get_os();
    let os: &str = &os_binding.as_str();

    match os {
        "windows" => {
            #[cfg(windows)]
            {
                let _ = download_latest_devkit_release_windows();

                let execution_path: String = format!("{}/devkitPro-Update.exe", current_dir().unwrap().to_string_lossy().to_string());
                let exe_ini_path: String = format!("{}/devkitProUpdate.ini", current_dir().unwrap().to_string_lossy().to_string());

                let _ = execute_updater_windows(&execution_path);

                println!("Cleaning devkitPro Updater...");
                let _ = remove_file(execution_path);
                let _ = remove_file(exe_ini_path);
                println!("Cleaned devkitPro Updater.");
                println!("devkitPro successfully updated.");
            }
            Ok(())
        },
        "unix" => {
            #[cfg(not(windows))]
            {
                todo!()
            }
            Ok(())
        },
        &_ => {
            Err(Box::new(CustomError::new(CustomErrorKind::UnsupportedOS, &format!("Unknown or unsuported OS {}!", os).to_string())))
        }
    }
}
