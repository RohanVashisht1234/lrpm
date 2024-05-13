pub mod database_functions;
pub mod commons_and_constants;
pub mod installer;

use clearscreen;
use colored::Colorize;
use std::io;
use database_functions::*;
use commons_and_constants::*;
use installer::*;

fn main() {
    clearscreen::clear().expect("failed to clear screen");

    println!(
        "{}",
        "Hello, Its lite-xl-rusty-plugins-manager!"
            .bold()
            .bright_blue()
    );
    println!("{}", display_commands().bold().bright_yellow());
    println!(
        "{} {} ",
        "What do you want to do?".cyan(),
        ">".bold().red()
    );

    let mut input_command = String::new();
    io::stdin().read_line(&mut input_command).unwrap();
    if input_command == "1\n".to_string() {
        installer(String::new());
    }
    return;
}
