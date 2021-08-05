use crate::common_errors::{CommandExecuteError, CommonError};
use crate::{constants, containers::common, helpers};
use clap::{AppSettings, Clap};
use std::{path::Path, process::Child, process::Command};
use users;

const DOCKER_EXEC: &str = "docker";

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Docker {}

impl Docker {
    fn _check_command(args: &[&str]) -> Result<bool, CommonError> {
        let output = Command::new(DOCKER_EXEC).args(args).output()?;

        if output.status.success() {
            let stdout = String::from_utf8(output.stdout)?;
            return Ok(stdout.lines().count() > 1);
        }

        let command = helpers::command_to_string(DOCKER_EXEC, &args);
        Err(CommandExecuteError::from_vec(command, output.stderr).into())
    }

    fn _exec_command(args: &[&str]) -> Result<Child, CommonError> {
        let command = Command::new(DOCKER_EXEC).args(args).spawn()?;
        Ok(command)
    }

    pub fn run_container(name: &str, homepath: &Path) -> Result<Child, CommonError> {
        let args = [
            "run",
            "-ti",
            "-e",
            "DISPLAY",
            "-e",
            "ICECC_SERVER",
            "-v=/tmp/.X11-unix:/tmp/.X11-unix",
            &format!(
                "-v=/run/user/{}/pulse:/run/user/1000/pulse:rw,z",
                users::get_current_uid()
            ),
            &format!("-v={}:/home/neon/kdepim:rw,z", homepath.display()),
            "--privileged",
            "--name",
            name,
            constants::DEFAULT_TAG,
        ];
        Self::_exec_command(&args)
    }

    pub fn start_detached_container(name: &str) -> Result<Child, CommonError> {
        let args = ["start", name];
        Self::_exec_command(&args)
    }

    pub fn start_attached_container(name: &str) -> Result<Child, CommonError> {
        let args = ["start", "-ai", name];
        Self::_exec_command(&args)
    }

    pub fn attach_container(name: &str) -> Result<Child, CommonError> {
        let args = ["attach", name];
        Self::_exec_command(&args)
    }

    pub fn exec_container(name: &str) -> Result<Child, CommonError> {
        let args = ["exec", "-it", "-u", "neon", name, "bash"];
        Self::_exec_command(&args)
    }

    pub fn lauch_in_container(name: &str, application: &str) -> Result<Child, CommonError> {
        let args = ["exec", "-d", "-u", "neon", name, application];
        Self::_exec_command(&args)
    }

    pub fn remove_container(name: &str) -> Result<Child, CommonError> {
        let args = ["rm", name];
        Self::_exec_command(&args)
    }

    pub fn stop_container(name: &str) -> Result<Child, CommonError> {
        let args = ["stop", name];
        Self::_exec_command(&args)
    }

    pub fn build_container() -> Result<Child, CommonError> {
        let args = ["build", "--no-cache", "--tag", constants::DEFAULT_TAG, "."];
        Self::_exec_command(&args)
    }

    pub fn check_container_exists(name: &str) -> Result<bool, CommonError> {
        let args = ["ps", "-a", "-f", &format!("name={}", name)];
        Self::_check_command(&args)
    }

    pub fn check_container_running(name: &str) -> Result<bool, CommonError> {
        let args = ["ps", "-f", &format!("name={}", name)];
        Self::_check_command(&args)
    }
}

impl common::ContainerOptions for Docker {
    fn run(&self, name: &str, attach: bool, homepath: &Path) -> Result<Child, CommonError> {
        if Self::check_container_running(name)? {
            if attach {
                return Self::attach_container(name);
            } else {
                return Self::exec_container(name);
            }
        } else {
            if Self::check_container_exists(name)? {
                return Self::start_attached_container(name);
            } else {
                return Self::run_container(name, homepath);
            }
        }
    }

    fn build(&self, name: &str) -> Result<Child, CommonError> {
        if Self::check_container_exists(name)? {
            let ans = helpers::prompt_y_n(
                "Do you want to destroy and recreate the existing kdepim:dev container?",
            )?;

            if ans {
                Self::stop_container(name)?.wait()?;
                Self::remove_container(name)?.wait()?;
            }
        }
        Self::build_container()
    }

    fn launch_gui(
        &self,
        name: &str,
        application: &str,
    ) -> Result<std::process::Child, CommonError> {
        if !Self::check_container_running(name)? {
            Self::start_detached_container(name)?.wait()?;
        }
        Self::lauch_in_container(name, application)
    }
}
