use crate::common_errors::CommonError;
use crate::constants::DEFAULT_CONTAINER_NAME;
use crate::containers::{
    common::ContainerOptions, configs::Config, docker::Docker, podman::Podman,
};
use clap::{AppSettings, Clap};
use std::path::{Path, PathBuf};
use std::process::Child;

/// This is a cli tool to manage containers created for kde development.
#[derive(Clap)]
#[clap(version = "1.0", author = "Ayush Singh <ayush1325@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Clap)]
enum Action {
    Build(Build),
    Run(Run),
    Config(Config),
    Launch(Launch),
}

/// Build the container image
#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Build {
    /// Name of container
    #[clap(short, long, default_value = DEFAULT_CONTAINER_NAME)]
    name: String,
    /// Build Image with Nvidia Support
    #[clap(long)]
    nvidia: bool,
    #[clap(subcommand)]
    container: ContainerType,
}

/// Run the kdepim container.
/// Creates a new container if it does not already exist.
#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Run {
    homepath: PathBuf,
    /// Name of container
    #[clap(short, long, default_value = DEFAULT_CONTAINER_NAME)]
    name: String,
    /// Attach to a running Container
    #[clap(short, long)]
    attach: bool,
    /// Nvidia Support
    #[clap(long)]
    nvidia: bool,
    #[clap(subcommand)]
    container: ContainerType,
}

/// Launch GUI application inside container.
#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Launch {
    /// Application to Launch
    application: String,
    /// Name of container
    #[clap(short, long, default_value = DEFAULT_CONTAINER_NAME)]
    name: String,
    #[clap(subcommand)]
    container: ContainerType,
}

#[derive(Clap)]
enum ContainerType {
    Docker(Docker),
    Podman(Podman),
}

impl ContainerOptions for ContainerType {
    fn run(
        &self,
        name: &str,
        attach: bool,
        homepath: &Path,
        nvidia: bool,
    ) -> Result<Child, CommonError> {
        match self {
            ContainerType::Docker(x) => x.run(name, attach, homepath, nvidia),
            ContainerType::Podman(x) => x.run(name, attach, homepath, nvidia),
        }
    }

    fn build(&self, name: &str, nvidia: bool) -> Result<Child, CommonError> {
        match self {
            ContainerType::Docker(x) => x.build(name, nvidia),
            ContainerType::Podman(x) => x.build(name, nvidia),
        }
    }

    fn launch_gui(
        &self,
        name: &str,
        application: &str,
    ) -> Result<std::process::Child, CommonError> {
        match self {
            ContainerType::Docker(x) => x.launch_gui(name, application),
            ContainerType::Podman(x) => x.launch_gui(name, application),
        }
    }
}

pub fn execute() -> Result<(), CommonError> {
    let opt: Opts = Opts::parse();

    match opt.action {
        Action::Build(x) => {
            x.container.build(&x.name, x.nvidia)?.wait()?;
        }
        Action::Run(x) => {
            x.container
                .run(&x.name, x.attach, &x.homepath, x.nvidia)?
                .wait()?;
        }
        Action::Config(x) => x.execute()?,
        Action::Launch(x) => {
            x.container.launch_gui(&x.name, &x.application)?.wait()?;
        }
    };

    Ok(())
}
