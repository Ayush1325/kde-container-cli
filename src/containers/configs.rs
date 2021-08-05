use crate::common_errors::CommonError;
use clap::{AppSettings, Clap};
use std::process::Command;

/// Extra configuration that might be needed.
#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Config {
    /// Add podman to xhost. Required to display GUI applications from podman contianer.
    #[clap(long)]
    xhost: bool,
}

impl Config {
    pub fn execute(&self) -> Result<(), CommonError> {
        if self.xhost {
            Self::add_podman_xhost()?;
        }
        Ok(())
    }

    fn add_podman_xhost() -> Result<(), CommonError> {
        println!("Adding to xhost...");
        Command::new("xhost")
            .arg("+local:podman@")
            .spawn()?
            .wait()?;
        Ok(())
    }
}
