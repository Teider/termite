#![feature(conservative_impl_trait)]

extern crate clap;

use std::io;
use std::io::prelude::*;
use clap::{Arg, App, SubCommand};

fn main() {
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
                          .version("0.0.1")
                          .subcommand(SubCommand::with_name("quit")
                                      .about("quit termite"))
                          .get_matches_from(parse_input(input.trim()));
    if let Some(_) = matches.subcommand_matches("quit") {
      break;
    }
  }
}

fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = &'a str> {
    Some("termite").into_iter().chain(input.split(' '))
}
