use crate::behaviour::Position;
use ed_balance::{IIndividual, IMutation};
use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap, hash::Hash, slice::Iter};

#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Clone, Copy)]
pub struct Mutation {
    pub first: Position,
    pub second: Position,
}

impl IMutation for Mutation {}

pub type Keys = HashMap<char, Position>;
pub type Score = (f64, u32, u32, u32, f64, f64);

#[derive(Debug, Clone)]
pub struct Keyboard {
    pub version: String,

    /// A list of 30 characters.\
    /// Each position represents a position on a keyboard.\
    /// 0-14 the left part, 15-29 the right part.\
    /// The numbering like in the `ergo-layout` app.\
    /// Right part mirrored left.
    /// `_` means a skipped and blocked key.
    pub keys: Keys,
    pub score: Score,

    pub mutations: Vec<Mutation>,
    pub parent_version: String,
    pub parent: Keys,
}

impl Keyboard {
    pub fn new(
        version: String,
        keys: Keys,
        score: Score,
        mutations: Vec<Mutation>,
        parent_version: String,
        parent: Keys,
    ) -> Box<Keyboard> {
        debug_assert_eq!(keys.len(), 26);
        debug_assert_eq!(keys.values().max().unwrap(), &29_u8);

        box_keyboard(Keyboard {
            version,
            keys,
            mutations,
            parent_version,
            parent,
            score,
        })
    }
}

impl PartialEq for Keyboard {
    fn eq(&self, other: &Self) -> bool {
        if self.keys.len() != other.keys.len() {
            return false;
        }

        for (c, p) in &self.keys {
            if let Some(other_value) = other.keys.get(c) {
                if other_value != p {
                    return false;
                }
            }
        }

        true
    }
}

impl Eq for Keyboard {}

impl Hash for Keyboard {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for (&c, &p) in self.keys.iter().sorted_by_key(|(&c, _)| c).into_iter() {
            c.hash(state);
            p.hash(state);
        }
    }
}

impl IIndividual<Mutation> for Keyboard {
    fn get_kind(&self) -> String {
        self.parent_version.clone()
    }

    fn to_string(&self) -> String {
        fn fill_missed_positions(iter: Iter<(char, u8)>, revert: bool) -> String {
            let inverted: HashMap<_, _> = iter.map(|(c, p)| (p % 5_u8, *c)).collect();

            (0_u8..=4_u8)
                .map(|i| if revert { 4 - i } else { i })
                .map(|i| match inverted.get(&i) {
                    Some(&c) => c,
                    _ => '_',
                })
                .join("")
        }

        let sorted = self
            .keys
            .iter()
            .sorted_by(|(_, i1), (_, i2)| i1.cmp(i2))
            .collect_vec();

        let left = sorted
            .iter()
            .filter(|(_, &p)| p < 15)
            .group_by(|(_, &p)| p / 5)
            .into_iter()
            .map(|(_, x)| {
                fill_missed_positions(x.map(|(&a, &b)| (a, b)).collect_vec().iter(), false)
            })
            .join(" ");

        let right = sorted
            .iter()
            .filter(|(_, &p)| p >= 15)
            .group_by(|(_, &p)| p / 5)
            .into_iter()
            .map(|(_, x)| {
                let sorted = x
                    .sorted_by(|(_, &i1), (_, &i2)| i2.cmp(&i1))
                    .map(|(&a, &b)| (a, b))
                    .collect_vec();

                fill_missed_positions(sorted.iter(), true)
            })
            .join(" ");

        let (effort, left_counter, right_counter, switch, left_effort, right_effort) = self.score;
        format!(
            "{}  {};{};{};{};{:.3};{:.2};{:.2};{:.3};{:.2}",
            left,
            right,
            left_counter,
            right_counter,
            switch,
            get_balance(left_counter as f64, right_counter as f64),
            left_effort,
            right_effort,
            get_factor(left_effort, right_effort),
            effort
        )
    }

    fn get_score(&self) -> f64 {
        self.score.0
    }
}

fn box_keyboard(keyboard: Keyboard) -> Box<Keyboard> {
    Box::new(keyboard)
}

/// better the ballance lower the factor.\
/// the ideal factor is 1 for the ideal balance (50x50).\
/// 1 means that the factor does not affect a score.
pub fn get_factor(left_score: f64, right_score: f64) -> f64 {
    let ballance = get_balance(left_score, right_score);

    // https://www.desmos.com/calculator
    // bigger power - less strict ballance
    3. - (2. / ((ballance - 1.).powi(2) + 1.))
}

fn get_balance(left_score: f64, right_score: f64) -> f64 {
    if left_score.partial_cmp(&right_score).unwrap() == Ordering::Greater {
        return left_score / right_score;
    }

    right_score / left_score
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn should_filter_unique() {
        let k1 = Keyboard {
            keys: [('a', 0_u8), ('b', 1_u8), ('c', 2_u8)]
                .iter()
                .cloned()
                .collect(),
            mutations: [Mutation {
                first: 1,
                second: 2,
            }]
            .to_vec(),
            parent: [('a', 0_u8), ('b', 1_u8), ('c', 2_u8)]
                .iter()
                .cloned()
                .collect(),
            parent_version: "parent_version".to_string(),
            score: (1., 1, 2, 3, 4., 5.),
            version: "version".to_string(),
        };

        let k2 = Keyboard {
            keys: [('a', 0_u8), ('b', 1_u8), ('c', 2_u8)]
                .iter()
                .cloned()
                .collect(),
            mutations: [Mutation {
                first: 3,
                second: 2,
            }]
            .to_vec(),
            parent: [('a', 0_u8)].iter().cloned().collect(),
            parent_version: "parent_version2".to_string(),
            score: (2., 3, 4, 5, 6., 7.),
            version: "version2".to_string(),
        };

        assert_eq!(&k1, &k2);

        let vec = [k1, k2].iter().cloned().unique().collect_vec();

        assert_eq!(vec.len(), 1);
    }
}
