use std::io::Error;
use std::process::Command;

pub fn open_terminal(path: &str) -> Result<(), Error>{
    if cfg!(target_os = "linux") {
        Command::new("gnome-terminal")
        .arg("--working-directory")
        .arg(path).output().expect("failed to execute process");
        Ok(())
    } else { Err(Error::new(std::io::ErrorKind::Other, "OS unsupported")) }
}