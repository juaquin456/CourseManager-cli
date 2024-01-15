use std::io::Error;
use std::process::Command;

/// Opens a terminal in the specified path
/// 
/// # Arguments
/// 
/// * `path` - The path to open the terminal in
/// 
/// # Returns
/// 
/// * `Result<(), Error>` - Returns an error if the OS is not supported
/// 
/// # Errors
/// 
/// * `Error` - The OS is not supported
///
pub fn open_terminal(path: &str) -> Result<(), Error>{
    if cfg!(target_os = "linux") {
        Command::new("gnome-terminal")
        .arg("--working-directory")
        .arg(path).output().expect("failed to execute process");
        Ok(())
    } else { Err(Error::new(std::io::ErrorKind::Other, "OS unsupported")) }
}