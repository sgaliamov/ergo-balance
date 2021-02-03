use super::{Behaviour, Position};
use crate::keyboard::{get_factor, Keys};
use itertools::Itertools;
use std::collections::HashMap;

/// lower score better because it shows less efforts and better ballance.
pub fn calculate_score(this: &Behaviour, keyboard: &Keys) -> (f64, u16, u16) {
    let (effort, left, right) = this
        .words
        .iter()
        .map(|x| calculate_word_score(this, keyboard, x))
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

    let factor = get_factor(left, right);
    let effort = effort * factor;

    (effort, left, right)
}

fn calculate_word_score(
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
            let both_left = a_is_left && b_is_left;
            let both_right = !a_is_left && !b_is_left;
            let same_part = both_left || both_right;

            if !same_part {
                let effort = behaviour.efforts[&key_b][&key_b];

                return (
                    behaviour.switch_penalty * effort,
                    both_left as u16,
                    both_right as u16,
                );
            }

            let effort = behaviour.efforts[&key_a][&key_b];

            if key_a == key_b {
                return (
                    effort * behaviour.same_key_penalty,
                    both_left as u16,
                    both_right as u16,
                );
            }

            (effort, both_left as u16, both_right as u16)
        })
        .fold(
            (0., 0_u16, 0_u16),
            |(total, left, right), (effort, both_left, both_right)| {
                (effort + total, left + both_left, right + both_right)
            },
        );

    (score + first, left, right)
}
