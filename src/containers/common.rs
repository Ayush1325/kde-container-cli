use crate::common_errors::CommonError;
use std::{path::Path, process};

pub trait ContainerOptions {
    fn create(
        &self,
        name: &str,
        homepath: &Path,
        nvidia: bool,
    ) -> Result<process::Child, CommonError>;

    fn enter(&self, name: &str, attach: bool) -> Result<process::Child, CommonError>;

    fn build(&self, name: &str, nvidia: bool) -> Result<process::Child, CommonError>;

    fn launch_gui(&self, name: &str, application: &str) -> Result<process::Child, CommonError>;
}
