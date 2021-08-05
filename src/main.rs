mod cli;
mod common_errors;
mod constants;
mod containers;
mod helpers;

fn main() {
    execute();
    // test();
}

fn execute() {
    if let Err(e) = cli::execute() {
        println!("{}", e);
    }
}

#[cfg(debug_assertions)]
fn test() {
    let t = helpers::prompt_y_n("Test").unwrap();
    println!("{}", t);
}
