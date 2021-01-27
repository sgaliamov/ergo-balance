mod calculation;

use calculation::{Behaviour, Letters, Mutation};
use ed_balance::{run, CliSettings};
use std::process;
use structopt::StructOpt;

// get a list of instances.
// do mutations. keep mutations as objects.
// calculate scores.
// get the bests mutations.
// cross best mutations.
// apply child mutations.

fn main() {
    let args = CliSettings::from_args();
    if let Err(e) = run::<Mutation, Letters, Behaviour>(args) {
        eprintln!("Calculations failed: {:#?}", e);
        process::exit(1);
    }
}
