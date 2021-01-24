use super::Behaviour;
use crate::keyboard::Keyboard;
use itertools::Itertools;
use std::collections::HashMap;

fn get_word_score(this: &Behaviour, keyboard: &HashMap<char, u8>, word: &str) -> f64 {
    let chars = word.chars().collect_vec();

    if chars.len() == 1 {
        let key = keyboard[&chars[0]];
        return this.efforts[&key][&key];
    }

    chars
        .iter()
        .tuple_windows()
        .map(|(a, b)| {
            let key_a = keyboard[a];
            let key_b = keyboard[b];
            let same_part = key_a >= 15 && key_b >= 15 || key_a < 15 && key_b < 15;

            if !same_part {
                return this.switch_penalty;
            }

            let effort = this.efforts[&key_a][&key_b];

            if key_a == key_b {
                return effort * this.same_key_penalty;
            }

            effort
        })
        .sum()
}

pub fn get_score(this: &Behaviour, individual: &Keyboard) -> f64 {
    this.words
        .iter()
        .map(|x| get_word_score(this, &individual.keys, x))
        .sum()
}
