use std::collections::HashMap;

use super::{score_calculator::get_score, Behaviour};
use crate::keyboard::Keyboard;
use ed_balance::get_version;
use itertools::Itertools;
use rand::prelude::SliceRandom;

pub fn generate(this: &Behaviour) -> Box<Keyboard> {
    let mut letters = ('a'..='z').collect_vec();
    letters.shuffle(&mut rand::thread_rng());

    let keys: HashMap<char, u8> = letters
        .into_iter()
        .enumerate()
        .map(|(i, e)| (e, i as u8))
        .collect();

    let version = get_version();

    Keyboard::new(
        version.clone(),
        keys.clone(),
        get_score(this, &keys),
        Vec::new(),
        version, // versions match to be able cross children with parents
        keys,
    )
}
