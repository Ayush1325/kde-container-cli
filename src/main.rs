mod cli;
mod common_errors;
mod constants;
mod containers;
mod helpers;

fn main() {
    execute();
}

fn execute() {
    if let Err(e) = cli::execute() {
        println!("{}", e);
    }
}

#[cfg(debug_assertions)]
fn test() {
    use std::process::Command;

    let t = [
        "--rm",
        "--security-opt=label=disable",
        "--hooks-dir=/usr/share/containers/oci/hooks.d/",
    ];

    Command::new("podman")
        .arg("run")
        .args(t)
        .arg("nvidia/cuda:11.0-base")
        .arg("nvidia-smi")
        .spawn()
        .unwrap()
        .wait();
}
