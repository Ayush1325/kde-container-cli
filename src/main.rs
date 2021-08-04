mod cli;
mod constants;
mod containers;
mod helpers;

fn main() {
    execute();
    // test();
}

fn execute() {
    match cli::execute() {
        Ok(mut x) => {
            if let Err(e) = x.wait() {
                println!("CLI exited unexpectedly:\n{}", e);
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    };
}

fn test() {
    let t = helpers::prompt_y_n("Test").unwrap();
    println!("{}", t);
}
