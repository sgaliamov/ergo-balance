mod generator;
mod loader;
mod model;
mod mutator;
mod recombination;
mod score_calculator;

use ed_balance::{get_version, CliSettings, Context, IBehaviour, IIndividual};
use io::BufRead;
use itertools::Itertools;
pub use model::*;
use std::{
    cmp::Ordering,
    fs::File,
    io::{self, Write},
};

use crate::keyboard::{Keyboard, Keys, Mutation};

impl IBehaviour<Mutation, Keyboard> for Behaviour {
    fn new(settings: &CliSettings) -> Self {
        loader::create(settings).expect("Failed to create the behaviour object.")
    }

    fn generate(&self) -> Box<Keyboard> {
        generator::generate(self)
    }

    fn calculate_score(&self, individual: &Keyboard) -> f64 {
        let (effort, _, _) = score_calculator::calculate_score(&self, &individual.keys);
        effort
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
        let (a_total, _, _) = a.score;
        let (b_total, _, _) = b.score;

        a_total.partial_cmp(&b_total).unwrap()
    }

    fn load(&self) -> std::io::Result<Vec<Box<Keyboard>>> {
        if let Ok(file) = File::open("data/keyboards.csv") {
            let lines = io::BufReader::new(file).lines();

            let keyboards = lines
                .map(|x| {
                    let line = x.unwrap();
                    let keys = line_to_keys(&line);
                    let score = score_calculator::calculate_score(&self, &keys);
                    let version = get_version();

                    Keyboard::new(
                        version.clone(),
                        keys.clone(),
                        score,
                        Vec::new(),
                        version,
                        keys,
                    )
                })
                .collect_vec();

            return Ok(keyboards);
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

fn line_to_keys(line: &str) -> Keys {
    let parts = line.split(';').collect_vec();
    let line = parts[0];
    let parts = line.split_whitespace().collect_vec();
    let left = parts
        .iter()
        .take(3)
        .flat_map(|part| part.chars())
        .enumerate()
        .map(|(p, c)| (c, p as u8));

    parts
        .iter()
        .skip(3)
        .flat_map(|part| part.chars().rev())
        .enumerate()
        .map(|(p, c)| (c, p as u8 + 15_u8))
        .merge(left)
        .filter(|(c, _)| c != &'_')
        .collect()
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
    fn test_line_to_keys() {
        let actual = line_to_keys("hntio asler zxcvd  wyfj_ qubpg km___; 5625.250; some text");
        let expected: Keys = [
            ('h', 0_u8),
            ('n', 1_u8),
            ('t', 2_u8),
            ('i', 3_u8),
            ('o', 4_u8),
            ('a', 5_u8),
            ('s', 6_u8),
            ('l', 7_u8),
            ('e', 8_u8),
            ('r', 9_u8),
            ('z', 10_u8),
            ('x', 11_u8),
            ('c', 12_u8),
            ('v', 13_u8),
            ('d', 14_u8),
            ('w', 19_u8),
            ('y', 18_u8),
            ('f', 17_u8),
            ('j', 16_u8),
            ('q', 24_u8),
            ('u', 23_u8),
            ('b', 22_u8),
            ('p', 21_u8),
            ('g', 20_u8),
            ('k', 29_u8),
            ('m', 28_u8),
        ]
        .iter()
        .cloned()
        .collect();

        assert_eq!(actual, expected);
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
            score: (0., 0, 0),
            version: "version".to_string(),
        };

        let actual = mutator::mutate(&behaviour, &individual);

        assert_eq!(actual.keys.len(), 3);
        assert_ne!(actual.keys, individual.keys);
        assert_eq!(actual.mutations.len(), behaviour.context.mutations_count);
    }
}
