use crate::common_errors::CommonError;
use std::{path::Path, process};

pub trait ContainerOptions {
    fn run(&self, name: &str, attach: bool, homepath: &Path)
        -> Result<process::Child, CommonError>;

    fn build(&self, name: &str) -> Result<process::Child, CommonError>;
}
