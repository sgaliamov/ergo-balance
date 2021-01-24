use super::{score_calculator::get_score, Behaviour};
use crate::keyboard::{Keyboard, Keys, Mutation};
use ed_balance::get_version;
use itertools::Itertools;
use rand::{prelude::SliceRandom, thread_rng};

pub fn mutate(this: &Behaviour, individual: &Keyboard) -> Box<Keyboard> {
    let mut rng = thread_rng();
    let mut mutations: Vec<Mutation> = Vec::with_capacity(this.context.mutations_count);
    let mut keys = individual
        .keys
        .iter()
        .filter(|(c, _)| !this.frozen_keys.contains_key(c))
        .map(|(&key, &value)| (key, value))
        .collect_vec();

    keys.shuffle(&mut rng);

    for index in 0..this.context.mutations_count {
        let second_index = keys.len() - index - 1;
        let (first_char, first) = keys[index];
        let (second_char, second) = keys[second_index];

        mutations.push(Mutation { first, second });
        keys[index] = (first_char, second);
        keys[second_index] = (second_char, first);
    }

    let version = get_version();
    let keys: Keys = keys.into_iter().collect();
    let score = get_score(this, &keys);

    Keyboard::new(
        version,
        keys.clone(),
        score,
        mutations,
        individual.version.clone(),
        individual.keys.clone(),
    )
}
