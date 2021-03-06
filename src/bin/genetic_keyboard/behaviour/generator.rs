use super::{score_calculator::calculate_score, Behaviour, FrozenKeys, Keyboard, Position};
use crate::keyboard::Keys;
use ed_balance::get_version;
use itertools::Itertools;
use rand::prelude::SliceRandom;
use std::collections::HashSet;

pub fn generate(this: &Behaviour) -> Box<Keyboard> {
    let version = get_version();
    let keys = generate_keys(&this.frozen_keys, &this.blocked_keys);

    debug_assert_eq!(keys.len(), 26);
    debug_assert_eq!(keys.values().max().unwrap(), &29_u8);

    Keyboard::new(
        version.clone(),
        keys.clone(),
        calculate_score(this, &keys),
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

    let frozen_values: HashSet<_> = frozen_keys.values().cloned().collect();
    let mut positions = (0..=29 as Position)
        .filter(|x| !blocked_keys.contains(x))
        .filter(|x| !frozen_values.contains(x))
        .collect_vec();
    positions.shuffle(rnd);

    letters
        .into_iter()
        .zip(positions.into_iter())
        .merge(frozen_keys.clone())
        .collect()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_generate_with_no_frozen() {
        let frozen_keys: FrozenKeys = [].iter().cloned().collect();
        let frozen_values: HashSet<_> = frozen_keys.values().cloned().collect();
        let blocked_keys: HashSet<Position> = [9, 14, 19, 24].iter().cloned().collect();

        let keys = generate_keys(&frozen_keys, &blocked_keys);

        let expected_keys = ('a'..='z').collect_vec();
        let actual_keys = keys
            .keys()
            .sorted_by(|a, b| a.cmp(b))
            .cloned()
            .collect_vec();

        let expected_values = (0..=29)
            .filter(|x| !blocked_keys.contains(x))
            .filter(|x| !frozen_values.contains(x))
            .merge(frozen_keys.values().cloned())
            .sorted_by(|a, b| a.cmp(b))
            .collect_vec();

        let actual_values = keys
            .values()
            .sorted_by(|a, b| a.cmp(b))
            .cloned()
            .collect_vec();

        assert_eq!(keys.len(), 26);
        assert_eq!(actual_keys, expected_keys);
        assert_eq!(actual_values, expected_values);
    }

    #[test]
    fn test_generate() {
        let frozen_keys: FrozenKeys = [('a', 1_u8), ('b', 2_u8), ('c', 29_u8)]
            .iter()
            .cloned()
            .collect();
        let frozen_values: HashSet<_> = frozen_keys.values().cloned().collect();
        let blocked_keys: HashSet<Position> = [0, 2, 15, 16, 17].iter().cloned().collect();

        let keys = generate_keys(&frozen_keys, &blocked_keys);

        let expected_keys = ('a'..='z').collect_vec();
        let actual_keys = keys
            .keys()
            .sorted_by(|a, b| a.cmp(b))
            .cloned()
            .collect_vec();

        let expected_values = (1..=29)
            .filter(|x| !blocked_keys.contains(x))
            .filter(|x| !frozen_values.contains(x))
            .merge(frozen_keys.values().cloned())
            .sorted_by(|a, b| a.cmp(b))
            .collect_vec();

        let actual_values = keys
            .values()
            .sorted_by(|a, b| a.cmp(b))
            .cloned()
            .collect_vec();

        assert_eq!(keys.len(), 26);
        assert_eq!(keys[&'a'], 1);
        assert_eq!(keys[&'b'], 2);
        assert_eq!(keys[&'c'], 29);
        assert_eq!(actual_keys, expected_keys);
        assert_eq!(actual_values, expected_values);
    }
}
