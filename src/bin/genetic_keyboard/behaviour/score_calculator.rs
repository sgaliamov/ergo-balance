use super::{Behaviour, Position};
use crate::keyboard::{get_factor, Keys, Score};
use itertools::Itertools;
use std::collections::HashMap;

/// lower score better because it shows less efforts and better ballance.
pub fn calculate_score(this: &Behaviour, keyboard: &Keys) -> Score {
    let (effort, left, right, switch, left_effort, right_effort) = this
        .words
        .iter()
        .map(|x| calculate_word_score(this, keyboard, x))
        .fold(
            (0., 0, 0, 0, 0., 0.),
            |(
                effort_total,
                left_total,
                right_total,
                switch_total,
                left_effort_total,
                right_effort_total,
            ),
             (
                word_effort,
                word_left,
                word_right,
                word_switch,
                word_left_effort,
                word_right_effort,
            )| {
                (
                    effort_total + word_effort,
                    left_total + word_left,
                    right_total + word_right,
                    switch_total + word_switch,
                    left_effort_total + word_left_effort,
                    right_effort_total + word_right_effort,
                )
            },
        );

    let factor = get_factor(left_effort, right_effort);
    let effort = effort * factor;

    (effort, left, right, switch, left_effort, right_effort)
}

fn calculate_word_score(
    behaviour: &Behaviour,
    keyboard: &HashMap<char, Position>,
    word: &str,
) -> Score {
    #[inline]
    fn is_left(position: Position) -> bool {
        position < 15
    }

    let chars = word.chars().collect_vec();
    let key = keyboard[&chars[0]];
    let first = behaviour.efforts[&key][&key]; // to count the score for the first or one letter
    let (score, left, right, switch, left_effort, right_effort) = chars
        .iter()
        .tuple_windows()
        .map(|(a, b)| {
            let key_a = keyboard[a];
            let key_b = keyboard[b];
            let a_is_left = is_left(key_a);
            let b_is_left = is_left(key_b);
            let both_left = a_is_left && b_is_left;
            let both_right = !a_is_left && !b_is_left;
            let switch = a_is_left != b_is_left;

            if switch {
                // key "a" is counted in a previous iteration,
                // so whe we have the hand switch we need to count effort on the second letters,
                // because the next hand "start" typing.
                let effort = behaviour.efforts[&key_b][&key_b];

                return (
                    behaviour.switch_penalty * effort,
                    both_left as u32,
                    both_right as u32,
                    switch as u32,
                    b_is_left,
                );
            }

            let effort = behaviour.efforts[&key_a][&key_b];

            if key_a == key_b {
                return (
                    effort * behaviour.same_key_penalty,
                    both_left as u32,
                    both_right as u32,
                    switch as u32,
                    b_is_left,
                );
            }

            (
                effort,
                both_left as u32,
                both_right as u32,
                switch as u32,
                b_is_left,
            )
        })
        .fold(
            (0., 0, 0, 0, 0., 0.),
            |(total, left, right, total_switch, left_effort, right_effort),
             (effort, both_left, both_right, switch, is_left)| {
                (
                    effort + total,
                    left + both_left,
                    right + both_right,
                    total_switch + switch,
                    left_effort + (if is_left { effort } else { 0. }),
                    right_effort + (if !is_left { effort } else { 0. }),
                )
            },
        );

    (
        score + first,
        left,
        right,
        switch,
        left_effort,
        right_effort,
    )
}
