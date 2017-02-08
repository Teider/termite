extern crate clap;

use std;
use std::io;
use std::io::prelude::*;
use std::sync::mpsc::Sender;
use self::clap::{App, AppSettings, SubCommand};

pub enum Command {
    Status,
}

pub fn init_shell(_: Sender<Command>) {
    println!("Starting termite:");
    let prompt = "λ ";
    loop {
        print!("{}", prompt);
        io::stdout().flush().ok().expect("Could not flush stdout");
        let mut input = String::new();
        if let Err(error) = io::stdin().read_line(&mut input) {
            println!("error: {}", error);
            break;
        }
        let matches = App::new("termite")
            .setting(AppSettings::NoBinaryName)
            .subcommand(SubCommand::with_name("quit").about("quit termite"))
            .get_matches_from_safe(parse_input(input.trim()));
        if let Err(_) = matches {
            println!("Invalid command");
            continue;
        }
        let matches = matches.unwrap();
        if let Some(_) = matches.subcommand_matches("quit") {
            println!("Exiting termite");
            break;
        }
    }
}

fn parse_input<'a>(input: &'a str) -> std::str::Split<'a, char> {
    input.split(' ')
}
