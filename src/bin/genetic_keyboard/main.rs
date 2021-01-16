mod behaviour;
mod keyboard;

use behaviour::Behaviour;
use ed_balance::{run, CliSettings};
use keyboard::{Keyboard, Mutation};
use std::process;
use structopt::StructOpt;

// 0. load texts in memory
// 1. create initial population
// 2. evaluate the population
// 3. take the best offspring
// 4. recombine
// 5. go to 2 till find the best

fn main() {
    let args = CliSettings::from_args();
    if let Err(e) = run::<Mutation, Keyboard, Behaviour>(args) {
        eprintln!("Calculations failed: {:#?}", e);
        process::exit(1);
    }
}
