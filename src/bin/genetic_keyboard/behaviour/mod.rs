mod generator;
mod loader;
mod model;
mod mutator;
mod recombination;
mod score_calculator;

use ed_balance::{CliSettings, Context, IBehaviour, IIndividual};
use io::BufRead;
use itertools::Itertools;
pub use model::*;
use std::{cmp::Ordering, fs::File, io::{self, Write}};

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

    fn score_cmp(&self, a: &Keyboard, b: &Keyboard) -> Ordering {
        let a_total = self.get_score(a);
        let b_total = self.get_score(b);

        a_total.partial_cmp(&b_total).unwrap()
    }

    fn load(&self) -> std::io::Result<Vec<Box<Keyboard>>> {
        if let Ok(file) = File::open("data/keyboards.csv"){
            let lines = io::BufReader::new(file).lines();

            score_calculator::get_score(&self, &individual.keys);

            Keyboard::new(version, keys, score, mutations, parent_version, parent)
        }

        Ok(Vec::new())
    }

    fn save(individuals: &Vec<Box<Keyboard>>) -> std::io::Result<()> {
        let text = individuals.iter().map(|x| x.to_string()).join("\n");
        let mut file = File::create("data/keyboards.csv")?;
        file.write_all(text.as_bytes())?;
        file.sync_all()?;

        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    fn default_context() -> Context {
        Context {
            mutations_count: 2,
            population_size: 10,
            children_count: 10,
            generations_count: 10,
            results_count: 10,
            left_count: 15,
            repeats_count: 10,
        }
    }

    fn default_efforts() -> Efforts {
        let efforts: Efforts = [
            (0, [(0, 1.), (1, 2.), (2, 3.)].iter().cloned().collect()),
            (1, [(0, 4.), (1, 5.), (2, 6.)].iter().cloned().collect()),
            (2, [(0, 7.), (1, 8.), (2, 9.)].iter().cloned().collect()),
        ]
        .iter()
        .cloned()
        .collect();

        efforts
    }

    fn default_behaviour() -> Behaviour {
        Behaviour {
            context: default_context(),
            blocked_keys: HashSet::new(),
            efforts: default_efforts(),
            frozen_keys: [('a', 1_u8)].iter().cloned().collect(),
            same_key_penalty: 2.,
            switch_penalty: 3.,
            words: ["abc".to_string()].to_vec(),
        }
    }

    #[test]
    fn should_mutate() {
        let behaviour = default_behaviour();
        let individual = Keyboard {
            keys: [('a', 0_u8), ('b', 1_u8), ('c', 2_u8)]
                .iter()
                .cloned()
                .collect(),
            mutations: Vec::new(),
            parent: HashMap::new(),
            parent_version: "parent_version".to_string(),
            score: 0.,
            version: "version".to_string(),
        };

        let actual = mutator::mutate(&behaviour, &individual);

        assert_eq!(actual.keys.len(), 3);
        assert_ne!(actual.keys, individual.keys);
        assert_eq!(actual.mutations.len(), behaviour.context.mutations_count);
    }
}
