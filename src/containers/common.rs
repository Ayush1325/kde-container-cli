use crate::common_errors::CommonError;
use std::{path::Path, process};

pub trait ContainerOptions {
    fn run(
        &self,
        name: &str,
        attach: bool,
        homepath: &Path,
        nvidia: bool,
    ) -> Result<process::Child, CommonError>;

    fn build(&self, name: &str, nvidia: bool) -> Result<process::Child, CommonError>;

    fn launch_gui(&self, name: &str, application: &str) -> Result<process::Child, CommonError>;
}
