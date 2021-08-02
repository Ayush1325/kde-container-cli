use clap::{AppSettings, Clap};

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
    /// Build the container image
    Build,
    /// Run the kdepim container.
    /// Creates a new container if it does not already exist.
    Run,
}

#[derive(Clap)]
enum ContainerType {
    Docker,
    Podman,
}

pub fn execute() {
    let opt = Opts::parse();
}
