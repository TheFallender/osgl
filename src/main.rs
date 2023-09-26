#![windows_subsystem = "windows"]

use std::env;
use std::process::Command;
use std::io;

fn main() -> io::Result<()> {
    // Set the environment variable
    env::set_var("OPENSSL_ia32cap", ":~0x20000000");

    // Attempt to launch the original executable
    match launch_original_exe() {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Failed to start original executable: {}", e.to_string());
            Err(e)
        }
    }
}

fn launch_original_exe() -> io::Result<()> {
    // Get the current executable path
    let current_exe_path = env::current_exe()?;
    let exe_name = current_exe_path
        .file_name()
        .ok_or(io::Error::new(io::ErrorKind::Other, "Failed to get executable name"))?
        .to_str()
        .ok_or(io::Error::new(io::ErrorKind::Other, "Failed to convert OsStr to str"))?;

    // Build the original executable name
    let original_exe = exe_name.replace(".exe", "") + "_original.exe";

    // Collect the command-line arguments
    let args: Vec<String> = env::args().skip(1).collect();

    // Spawn the original executable with the collected arguments
    Command::new(format!("./{}", original_exe))
        .args(&args)
        .spawn()?;

    Ok(())
}
