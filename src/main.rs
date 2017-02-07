extern crate clap;
extern crate time;

use std::io;
use std::io::prelude::*;
use std::sync::mpsc::{channel, Sender, TryRecvError};
use std::thread;
use clap::{Arg, App, AppSettings, SubCommand};
use time::{Duration, PreciseTime};

const TICKS_PER_SECOND: i32 = 5;

fn main() {
    let (tx, rx) = channel();
    thread::spawn(move || { init_shell(tx); });
    let mut tick_count: u64 = 0;
    loop {
        let frame_begin = PreciseTime::now();
        let result = rx.try_recv();
        if let Err(TryRecvError::Disconnected) = result {
            break;
        }
        let frame_duration = frame_begin.to(PreciseTime::now());
        let idle_time = (Duration::seconds(1) / TICKS_PER_SECOND) - frame_duration;
        if let Ok(std_duration) = idle_time.to_std() {
            thread::sleep(std_duration);
        }
        tick_count += 1;
    }
}

fn init_shell(tx: Sender<u64>) {
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
            //tx.send(0);
            break;
        }
    }
}

fn parse_input<'a>(input: &'a str) -> std::str::Split<'a, char> {
    input.split(' ')
}
