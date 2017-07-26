extern crate time;
extern crate rand;

mod shell;
mod grid;

use std::sync::mpsc::{channel, TryRecvError};
use std::thread;
use time::{Duration, PreciseTime};

const TICKS_PER_SECOND: i32 = 5;

fn main() {
    let grid: grid::Grid = grid::GridBuilder::new(4, 4).origin((1, 1)).build();
    print!("Generated grid:\n{}\n", grid);
    let (tx, rx) = channel();
    thread::spawn(move || { shell::init_shell(tx); });
    let mut tick_count: u64 = 0;
    'main: loop {
        let frame_begin = PreciseTime::now();
        loop {
            match rx.try_recv() {
                Ok(command) => consume_command(command),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => break 'main,
            }
        }
        let frame_duration = frame_begin.to(PreciseTime::now());
        let idle_time = (Duration::seconds(1) / TICKS_PER_SECOND) - frame_duration;
        if let Ok(std_duration) = idle_time.to_std() {
            thread::sleep(std_duration);
        }
        tick_count += 1;
    }
}

fn consume_command(_: shell::Command) {}
