mod generator;
mod loader;
mod model;
mod mutator;
mod recombination;
mod score_calculator;

use ed_balance::{CliSettings, Context, IBehaviour};
pub use model::*;

use crate::keyboard::{Keyboard, Mutation};

impl IBehaviour<Mutation, Keyboard> for Behaviour {
    fn new(settings: &CliSettings) -> Self {
        loader::create(settings).unwrap()
    }

    fn generate(&self) -> Box<Keyboard> {
        generator::generate(self)
    }

    fn get_score(&self, individual: &Keyboard) -> f64 {
        score_calculator::get_score(&self, &individual.keys)
    }

    fn cross(&self, individual: &Keyboard, partner: &Keyboard) -> Box<Keyboard> {
        recombination::cross(self, individual, partner)
    }

    fn mutate(&self, individual: &Keyboard) -> Box<Keyboard> {
        mutator::mutate(self, individual)
    }

    fn get_context<'a>(&'a self) -> &'a Context {
        &self.context
    }
}
