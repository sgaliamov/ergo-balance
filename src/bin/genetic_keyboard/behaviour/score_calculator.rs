use super::{Behaviour, Position};
use crate::keyboard::Keys;
use itertools::Itertools;
use std::collections::HashMap;

fn get_word_score(behaviour: &Behaviour, keyboard: &HashMap<char, Position>, word: &str) -> f64 {
    let chars = word.chars().collect_vec();

    let key = keyboard[&chars[0]];
    let first = behaviour.efforts[&key][&key];
    let sum: f64 = chars
        .iter()
        .tuple_windows()
        .map(|(a, b)| {
            let key_a = keyboard[a];
            let key_b = keyboard[b];
            let same_part = key_a >= 15_u8 && key_b >= 15_u8 || key_a < 15_u8 && key_b < 15_u8;

            if !same_part {
                return behaviour.switch_penalty;
            }

            let effort = behaviour.efforts[&key_a][&key_b];

            if key_a == key_b {
                return effort * behaviour.same_key_penalty;
            }

            effort
        })
        .sum();

    sum + first
}

pub fn get_score(this: &Behaviour, keyboard: &Keys) -> f64 {
    this.words
        .iter()
        .map(|x| get_word_score(this, keyboard, x))
        .sum()
}
