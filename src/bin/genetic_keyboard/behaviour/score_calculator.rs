use super::Behaviour;
use crate::keyboard::Keys;
use itertools::Itertools;
use std::collections::HashMap;

fn get_word_score(behaviour: &Behaviour, keyboard: &HashMap<char, u8>, word: &str) -> f64 {
    let chars = word.chars().collect_vec();

    if chars.len() == 1 {
        let key = keyboard[&chars[0]];
        return behaviour.efforts[&key][&key];
    }

    chars
        .iter()
        .tuple_windows()
        .map(|(a, b)| {
            let key_a = keyboard[a];
            let key_b = keyboard[b];
            let same_part = key_a >= 15 && key_b >= 15 || key_a < 15 && key_b < 15;

            if !same_part {
                return behaviour.switch_penalty;
            }

            let effort = behaviour
                .efforts
                .get(&key_a)
                .unwrap_or_else(|| panic!("Can not find nested map for key {}", key_a))
                .get(&key_b)
                .unwrap_or_else(|| panic!("Can not find effort for key {}", key_b))
                .to_owned();

            if key_a == key_b {
                return effort * behaviour.same_key_penalty;
            }

            effort
        })
        .sum()
}

pub fn get_score(this: &Behaviour, keyboard: &Keys) -> f64 {
    this.words
        .iter()
        .map(|x| get_word_score(this, keyboard, x))
        .sum()
}
