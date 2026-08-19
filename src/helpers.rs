use std::io::{Error, ErrorKind, Result, Write};
use std::fs::{File, create_dir, remove_file};
use std::path::Path;
use std::env::current_dir;
use std::process::Command;

use crate::cli::NewOptions;
use crate::code_templates::{C_CXX_CODE, MAKEFILE_CODE};
use crate::error::{CustomError, CustomErrorKind};
use crate::handlers::{build_path_from_cwd, build_path_from_two, get_os, System};

#[cfg(windows)]
use crate::handlers::{download_latest_devkit_release_windows, execute_updater_windows};

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

    let system_os: System = get_os();
    let os_family: &str = &system_os.family.as_str();
    let os: &str = &system_os.os.as_str();

    println!("Detected System: {}-like {}", os_family, os);

    match os_family {
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
                let unix_download_url: &str = "https://github.com/devkitPro/pacman/releases/download/v6.0.2/devkitpro-pacman-installer.pkg";
                let debian_download_url: &str = "https://apt.devkitpro.org/install-devkitpro-pacman";

                if os == "debian" {
                    println!("Using Updater/Installer from {}.", debian_download_url);
                    let _ = Command::new("wget")
                        .arg(debian_download_url)
                        .spawn()
                        .expect(Err(CustomError::new(CustomErrorKind::WgetDownloadFail, &format!("Something went wrong while downloading {}!", debian_download_url).to_string()))?);
                } else {
                    println!("Using Updater/Installer from {}.", unix_download_url);
                    let _ = Command::new("wget")
                        .args(["-qO", "devkitPro-Pacman.pkg", unix_download_url])
                        .spawn()
                        .expect(Err(CustomError::new(CustomErrorKind::WgetDownloadFail, &format!("Something went wrong while downloading {}!", unix_download_url).to_string()))?);
                }

                if os == "macos" {
                    let _ = Command::new("sudo")
                        .args(["installer", "-pkg", format!("{:?}/devkitPro-Pacman.pkg", current_dir()).as_str(), "-target", "/"])
                        .spawn()
                        .expect(Err(CustomError::new(CustomErrorKind::MacOSInstallerFail, "Something went wrong with the MacOS .pkg Installer!"))?);
                } else if os == "android" {
                    let _ = Command::new("chmod")
                        .args(["+x", "./dvkitPro-Pacman"])
                        .spawn()
                        .expect(Err(CustomError::new(CustomErrorKind::UnixScriptRightsFail, "Something went wrong with the rights (chmod) for the UNIX Install script!"))?);
                    let _ = Command::new("./devkitPro-Pacman")
                        .spawn()
                        .expect(Err(CustomError::new(CustomErrorKind::UnixInstallScriptFail, "Something went wrong with the UNIX install script!"))?);
                } else {
                    let _ = Command::new("chmod")
                        .args(["+x", "./dvkitPro-Pacman"])
                        .spawn()
                        .expect(Err(CustomError::new(CustomErrorKind::UnixScriptRightsFail, "Something went wrong with the rights (chmod) for the UNIX Install script!"))?);
                    let _ = Command::new("sudo")
                        .arg("./devkitPro-Pacman")
                        .spawn()
                        .expect(Err(CustomError::new(CustomErrorKind::UnixInstallScriptFail, "Something went wrong with the UNIX install script!"))?);
                }
            }
            Ok(())
        },
        &_ => {
            Err(Box::new(CustomError::new(CustomErrorKind::UnsupportedOS, &format!("Unknown or unsuported OS {}!", os).to_string())))
        }
    }
}
