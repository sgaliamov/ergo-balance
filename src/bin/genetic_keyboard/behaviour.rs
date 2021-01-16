use ed_balance::{Context, IBehaviour};

use crate::keyboard::{Keyboard, Mutation};

// use super::{Letters, LettersPointer, Mutation};
// use ed_balance::{
//     get_score, get_version,
//     models::{CliSettings, Digraphs},
//     Context, IBehaviour,
// };
// use itertools::{min, Itertools};
// use rand::{prelude::SliceRandom, thread_rng};

pub struct Behaviour {
    pub context: Context,
}

impl IBehaviour<Mutation, Keyboard> for Behaviour {
    fn new(settings: &ed_balance::CliSettings) -> Self {
        todo!()
    }

    fn generate(&self) -> Box<Keyboard> {
        todo!()
    }

    fn get_score(&self, individual: &Keyboard) -> f64 {
        todo!()
    }

    fn cross(&self, individual: &Keyboard, partner_mutations: &Vec<Mutation>) -> Box<Keyboard> {
        todo!()
    }

    fn mutate(&self, individual: &Keyboard) -> Box<Keyboard> {
        todo!()
    }

    fn get_context<'a>(&'a self) -> &'a Context {
        todo!()
    }
}
