use std::env::current_dir;
use std::env::consts::OS;
use std::process::Command;
use std::error::Error;
use std::io::Result;

use windows::core::PCWSTR;
use windows::Win32::System::Threading::{WaitForSingleObject, INFINITE};
use windows::Win32::UI::Shell::{ShellExecuteExW, SEE_MASK_NOCLOSEPROCESS, SHELLEXECUTEINFOW};
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

use crate::error::{CustomErrorKind, CustomError};

pub fn build_path_from_cwd(name: &str) -> String {
    let cwd = current_dir();
    return cwd.unwrap().to_string_lossy().to_string() + "/" + name;
}

pub fn build_path_from_two(first: &str, second: &str) -> String {
    return first.to_string() + "/" + second;
}

pub fn get_os() -> String {
    OS.to_string()
}

#[cfg(windows)]
pub fn download_latest_devkit_release_windows() -> core::result::Result<(), Box<dyn Error>> {
    let updater_url: &str = "https://github.com/devkitPro/installer/releases/download/v3.0.3/devkitProUpdater-3.0.3.exe";

    println!("Using devvkitPro Updater from {:?}", updater_url);

    let _ = Command::new("wget")
        .args(["-qO", "devkitPro-Update.exe", updater_url])
        .status()
        .expect(Err(CustomError::new(CustomErrorKind::FailedDownload, &format!("Failed to download Updater from {}", updater_url).to_string()))?);

    println!("Downloaded Updater from {:?}", updater_url);

    Ok(())
}

#[cfg(windows)]
pub fn execute_updater_windows(execution_path: &String) -> Result<()> {
    let path: &str = execution_path.as_str();
    let path_wide: Vec<u16> = path.encode_utf16().chain(Some(0)).collect();
    let verb_wide: Vec<u16> = "runas".encode_utf16().chain(Some(0)).collect();

    let mut exec_info: SHELLEXECUTEINFOW = SHELLEXECUTEINFOW {
        cbSize: std::mem::size_of::<SHELLEXECUTEINFOW>() as u32,
        fMask: SEE_MASK_NOCLOSEPROCESS,
        lpVerb: PCWSTR(verb_wide.as_ptr()),
        lpFile: PCWSTR(path_wide.as_ptr()),
        nShow: SW_SHOWNORMAL.0,
        ..Default::default()
    };

    unsafe {
        ShellExecuteExW(&mut exec_info)?;

        WaitForSingleObject(exec_info.hProcess, INFINITE);
    }

    Ok(())
}
