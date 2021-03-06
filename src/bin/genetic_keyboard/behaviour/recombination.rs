use super::{score_calculator::calculate_score, Behaviour};
use crate::keyboard::{Keyboard, Keys};
use ed_balance::get_version;
use itertools::Itertools;
use rand::prelude::SliceRandom;
use std::collections::HashMap;

pub fn cross(this: &Behaviour, individual: &Keyboard, partner: &Keyboard) -> Box<Keyboard> {
    let mut keys: HashMap<_, _> = individual
        .parent
        .iter()
        .map(|(key, value)| (value, key))
        .collect();

    let mut mutations: Vec<_> = individual
        .mutations
        .iter()
        .chain(partner.mutations.iter())
        .unique()
        .map(|&x| x)
        .collect();

    mutations.shuffle(&mut rand::thread_rng());

    for mutation in mutations.iter().take(this.context.mutations_count) {
        let first_char = keys[&mutation.first];
        let second_char = keys[&mutation.second];
        *keys.entry(&mutation.first).or_insert(second_char) = second_char;
        *keys.entry(&mutation.second).or_insert(first_char) = first_char;
    }

    let keys: Keys = keys
        .into_iter()
        .map(|(&key, &value)| (value, key))
        .collect();

    let score = calculate_score(this, &keys);

    debug_assert_eq!(keys.len(), 26);
    debug_assert_eq!(keys.values().max().unwrap(), &29_u8);

    Keyboard::new(
        get_version(),
        keys,
        score,
        mutations,
        individual.parent_version.clone(),
        individual.parent.clone(),
    )
}
