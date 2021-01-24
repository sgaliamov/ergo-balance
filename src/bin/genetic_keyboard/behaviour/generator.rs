use super::{score_calculator::get_score, Behaviour};
use crate::keyboard::{Keyboard, Keys};
use ed_balance::get_version;
use itertools::Itertools;
use rand::prelude::SliceRandom;

pub fn generate(this: &Behaviour) -> Box<Keyboard> {
    let rnd = &mut rand::thread_rng();

    let mut letters = ('a'..='z')
        .filter(|x| !this.frozen_keys.contains_key(x))
        .collect_vec();
    letters.shuffle(rnd);

    let mut positions = (0..30 as u8)
        .filter(|x| !this.blocked_keys.contains(x))
        .collect_vec();
    positions.shuffle(rnd);

    let mut keys: Keys = letters.into_iter().zip(positions.into_iter()).collect();
    keys.extend(&this.frozen_keys);
    debug_assert_eq!(keys.len(), 26);
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
