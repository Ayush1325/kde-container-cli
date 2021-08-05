use std::io::{self, Write};

pub fn prompt_y_n(msg: &str) -> Result<bool, io::Error> {
    loop {
        print!("{} [y/n] ", msg);
        io::stdout().flush()?;
        let mut inp = String::new();
        io::stdin().read_line(&mut inp)?;
        match inp.trim() {
            "y" => return Ok(true),
            "n" => return Ok(false),
            _ => print!("Invalid Input\n"),
        }
    }
}

pub fn command_to_string(command: &str, args: &[&str]) -> String {
    let combined_args = args.join(" ");
    format!("{} {}", command, combined_args)
}
