#![feature(conservative_impl_trait)]

extern crate clap;

use std::io;
use std::io::prelude::*;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use clap::{Arg, App, AppSettings, SubCommand};

fn main() {
    let (tx, rx) = channel();
    thread::spawn(move || {
        init_shell(tx);
    });
    loop {
        rx.recv().unwrap();
        break;
    }
}

fn init_shell(tx: Sender<u64>) {
  println!("Starting termite:");
  let prompt = "λ ";
  loop {
    print!("{}", prompt);
    io::stdout().flush().ok().expect("Could not flush stdout");
    let mut input = String::new();
    if let Err(error) =  io::stdin().read_line(&mut input) {
      println!("error: {}", error);
      break
    }
    let matches = App::new("termite")
                          .setting(AppSettings::NoBinaryName)
                          .subcommand(SubCommand::with_name("quit")
                                      .about("quit termite"))
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

fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = &'a str> {
    input.split(' ')
}
