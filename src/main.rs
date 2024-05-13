pub mod database_functions;
pub mod commons_and_constants;
pub mod installer;
pub mod downloaders_and_setters;


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
    if input_command == *"1\n" {
        println!("{}", "Which plugin do you want to install? >".bold().bright_yellow());
        let mut input_plugin_name = String::new();
        io::stdin().read_line(&mut input_plugin_name).unwrap();
        installer(input_plugin_name);
    }
}
