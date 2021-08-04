use crate::containers::common;
use clap::{AppSettings, Clap};
use std::{path::Path, process::Command};
use users;

const PODMAN_EXEC: &str = "podman";

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Podman {}

impl Podman {
    fn _check_command(args: &[&str]) -> Result<bool, common::CommonError> {
        let output = Command::new(PODMAN_EXEC).args(args).output()?;

        if output.status.success() {
            let stdout = String::from_utf8(output.stdout)?;
            return Ok(stdout.lines().count() > 1);
        }

        let command = common::command_to_string(PODMAN_EXEC, &args);
        Err(common::CommandExecuteError::from_vec(command, output.stderr).into())
    }

    fn _exec_command(args: &[&str]) -> Result<std::process::Child, common::CommonError> {
        let command = Command::new(PODMAN_EXEC).args(args).spawn()?;
        Ok(command)
    }

    pub fn run_container(
        name: &str,
        homepath: &Path,
    ) -> Result<std::process::Child, common::CommonError> {
        let args = [
            "run",
            "-ti",
            "--net=host",
            "-e",
            "DISPLAY",
            "-e",
            "ICECC_SERVER",
            "-v=/tmp/.X11-unix:/tmp/.X11-unix",
            &format!(
                "-v=/run/user/{}/pulse:/run/user/1000/pulse",
                users::get_current_uid()
            ),
            &format!("-v={}:/home/neon/kdepim", homepath.display()),
            "--privileged",
            "--name",
            name,
            "kdepim:dev",
        ];
        Self::_exec_command(&args)
    }

    pub fn start_container(name: &str) -> Result<std::process::Child, common::CommonError> {
        let args = ["start", "-ai", name];
        Self::_exec_command(&args)
    }

    pub fn attach_container(name: &str) -> Result<std::process::Child, common::CommonError> {
        let args = ["attach", name];
        Self::_exec_command(&args)
    }

    pub fn exec_container(name: &str) -> Result<std::process::Child, common::CommonError> {
        let args = ["exec", "-it", "-u", "neon", name, "bash"];
        Self::_exec_command(&args)
    }

    pub fn check_container_exists(name: &str) -> Result<bool, common::CommonError> {
        let args = ["ps", "-a", "-f", &format!("name={}", name)];
        Self::_check_command(&args)
    }

    pub fn check_container_running(name: &str) -> Result<bool, common::CommonError> {
        let args = ["ps", "-f", &format!("name={}", name)];
        Self::_check_command(&args)
    }
}

impl common::ContainerOptions for Podman {
    fn run(
        &self,
        name: &str,
        attach: bool,
        homepath: &Path,
    ) -> Result<std::process::Child, common::CommonError> {
        if Self::check_container_running(name)? {
            if attach {
                return Self::attach_container(name);
            } else {
                return Self::exec_container(name);
            }
        } else {
            if Self::check_container_exists(name)? {
                return Self::start_container(name);
            } else {
                return Self::run_container(name, homepath);
            }
        }
    }
}
