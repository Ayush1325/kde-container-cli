pub mod cli;
pub mod constants;
mod containers;

fn main() {
    match cli::execute() {
        Ok(mut x) => {
            x.wait();
        }
        Err(e) => {
            println!("{}", e);
        }
    };
    // test();
}

fn test() {
    use std::env;
    use std::process::Command;
    use users;

    let uid = users::get_current_uid();
    println!("{}", uid);
}
