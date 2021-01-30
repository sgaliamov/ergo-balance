use super::{Behaviour, Position};
use crate::keyboard::Keys;
use itertools::Itertools;
use std::collections::HashMap;

pub fn get_score(this: &Behaviour, keyboard: &Keys) -> (f64, u16, u16) {
    let (effort, left, right) = this
        .words
        .iter()
        .map(|x| get_word_score(this, keyboard, x))
        .fold(
            (0., 0, 0),
            |(effort_total, left_total, right_total), (effort, left, right)| {
                (
                    effort_total + effort,
                    left + left_total,
                    right + right_total,
                )
            },
        );

    // todo: use factor
    (effort, left, right)
}

fn get_word_score(
    behaviour: &Behaviour,
    keyboard: &HashMap<char, Position>,
    word: &str,
) -> (f64, u16, u16) {
    #[inline]
    fn is_left(position: Position) -> bool {
        position < 15
    }

    let chars = word.chars().collect_vec();
    let key = keyboard[&chars[0]];
    let first = behaviour.efforts[&key][&key];
    let (score, left, right) = chars
        .iter()
        .tuple_windows()
        .map(|(a, b)| {
            let key_a = keyboard[a];
            let key_b = keyboard[b];
            let a_is_left = is_left(key_a);
            let b_is_left = is_left(key_b);
            let same_part = !a_is_left && !b_is_left || a_is_left && b_is_left;

            if !same_part {
                return (behaviour.switch_penalty, b_is_left);
            }

            let effort = behaviour.efforts[&key_a][&key_b];

            if key_a == key_b {
                return (effort * behaviour.same_key_penalty, b_is_left);
            }

            (effort, b_is_left)
        })
        .fold(
            (0., 0_u16, 0_u16),
            |(total, left, right), (effort, is_left)| {
                (
                    effort + total,
                    left + is_left as u16,
                    right + !is_left as u16,
                )
            },
        );

    (
        score + first,
        left + is_left(key) as u16,
        right + !is_left(key) as u16,
    )
}

// fn get_factor(left_score: f64, right_score: f64) -> f64 {
//     let factor = if left_score.partial_cmp(&right_score).unwrap() == Ordering::Less {
//         left_score / right_score
//     } else {
//         right_score / left_score
//     };

//     1.1 - 0.1 / factor
// }
