extern crate clap;
extern crate rustyline;

use std;
use std::sync::mpsc::Sender;
use self::clap::{App, AppSettings, SubCommand};
use self::rustyline::Editor;
use self::rustyline::error::ReadlineError;

pub enum Command {
    Status,
}

const PROMPT: &'static str = "λ ";

pub fn init_shell(_: Sender<Command>) {
    println!("Starting termite:");
    let mut rl = Editor::<()>::new();
    loop {
        let input = rl.readline(PROMPT);
        if let Err(err) = input {
            match err {
                ReadlineError::Interrupted => break,
                ReadlineError::Eof => break,
                _ =>  {
                    println!("Error: {:?}", err);
                    break
                }
            }
        }
        let input = input.unwrap();
        rl.add_history_entry(&input);
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
