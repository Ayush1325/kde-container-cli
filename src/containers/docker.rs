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

    pub fn create_container(
        name: &str,
        homepath: &Path,
        nvidia: bool,
    ) -> Result<Child, CommonError> {
        let pulse_mount = format!(
            "-v=/run/user/{}/pulse:/run/user/1000/pulse",
            users::get_current_uid()
        );
        let home_mount = format!("-v={}:/home/neon/kdepim", homepath.display());

        let mut args = vec!["run"];

        if nvidia {
            args.extend_from_slice(&["--gpus", "all"]);
        } else {
            args.push("--privileged");
        }

        args.extend_from_slice(&[
            "-ti",
            "--net=host",
            "-e",
            "DISPLAY",
            "-e",
            "ICECC_SERVER",
            "-v=/tmp/.X11-unix:/tmp/.X11-unix",
            &pulse_mount,
            &home_mount,
            "--name",
            name,
            constants::DEFAULT_TAG,
        ]);

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

    pub fn build_container(nvidia: bool) -> Result<Child, CommonError> {
        let args = if nvidia {
            [
                "build",
                "--no-cache",
                "--tag",
                constants::DEFAULT_TAG,
                "-f",
                constants::DOCKERFILE_NVIDIA,
            ]
        } else {
            [
                "build",
                "--no-cache",
                "--tag",
                constants::DEFAULT_TAG,
                "-f",
                constants::DOCKERFILE_NORMAL,
            ]
        };
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
    fn create(&self, name: &str, homepath: &Path, nvidia: bool) -> Result<Child, CommonError> {
        Self::create_container(name, homepath, nvidia)
    }

    fn enter(&self, name: &str, attach: bool) -> Result<Child, CommonError> {
        if Self::check_container_running(name)? {
            if attach {
                Self::attach_container(name)
            } else {
                Self::exec_container(name)
            }
        } else {
            if Self::check_container_exists(name)? {
                Self::start_attached_container(name)
            } else {
                Err(CommonError::EarlyExit(
                    "Contaner Does not Exist".to_string(),
                ))
            }
        }
    }

    fn build(&self, name: &str, nvidia: bool) -> Result<Child, CommonError> {
        if Self::check_container_exists(name)? {
            let ans = helpers::prompt_y_n(
                "Do you want to destroy and recreate the existing kdepim:dev container?",
            )?;

            if ans {
                Self::stop_container(name)?.wait()?;
                Self::remove_container(name)?.wait()?;
            } else {
                return Err(CommonError::EarlyExit(String::from("")));
            }
        }
        Self::build_container(nvidia)
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
