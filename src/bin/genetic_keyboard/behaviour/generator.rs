use super::{score_calculator::get_score, Behaviour, FrozenKeys, Keyboard, Position};
use crate::keyboard::Keys;
use ed_balance::get_version;
use itertools::Itertools;
use rand::prelude::SliceRandom;
use std::collections::HashSet;

pub fn generate(this: &Behaviour) -> Box<Keyboard> {
    let version = get_version();
    let keys = generate_keys(&this.frozen_keys, &this.blocked_keys);

    Keyboard::new(
        version.clone(),
        keys.clone(),
        get_score(this, &keys),
        Vec::new(),
        version, // versions match to be able cross children with parents
        keys,
    )
}

fn generate_keys(frozen_keys: &FrozenKeys, blocked_keys: &HashSet<Position>) -> Keys {
    let rnd = &mut rand::thread_rng();
    let mut letters = ('a'..='z')
        .filter(|x| !frozen_keys.contains_key(x))
        .collect_vec();
    letters.shuffle(rnd);

    let mut positions = (0..30 as Position)
        .filter(|x| !blocked_keys.contains(x))
        .collect_vec();
    positions.shuffle(rnd);

    let mut keys: Keys = letters.into_iter().zip(positions.into_iter()).collect();
    keys.extend(frozen_keys);
    debug_assert_eq!(keys.len(), 26);

    keys
}

// #[cfg(test)]
// pub mod tests {
//     use super::*;

//     #[test]
//     fn test_generate() {
//         generate_keys(this)
//     }
// }
