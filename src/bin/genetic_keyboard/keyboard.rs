use crate::behaviour::Position;
use ed_balance::{IIndividual, IMutation};
use itertools::Itertools;
use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Clone, Copy)]
pub struct Mutation {
    pub first: Position,
    pub second: Position,
}

impl IMutation for Mutation {}

pub type Keys = HashMap<char, Position>;

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
    pub score: f64,

    pub mutations: Vec<Mutation>,
    pub parent_version: String,
    pub parent: Keys,
}

impl Keyboard {
    pub fn new(
        version: String,
        keys: Keys,
        score: f64,
        mutations: Vec<Mutation>,
        parent_version: String,
        parent: Keys,
    ) -> Box<Keyboard> {
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
        let sorted = self
            .keys
            .iter()
            .sorted_by(|(_, i1), (_, i2)| i1.cmp(i2))
            .collect_vec();

        let left = sorted
            .iter()
            .take(15)
            .group_by(|(_, &p)| p / 5)
            .into_iter()
            .map(|(_, x)| x.map(|(c, _)| c).join(""))
            .join(" ");

        let right = sorted
            .iter()
            .skip(15)
            .group_by(|(_, &p)| p / 5)
            .into_iter()
            .map(|(_, x)| {
                x.sorted_by(|(_, &i1), (_, &i2)| i2.cmp(&i1))
                    .map(|(c, _)| c)
                    .join("")
            })
            .join(" ");

        format!("{}  {}; {:.3};", left, right, self.score)
    }
}

fn box_keyboard(keyboard: Keyboard) -> Box<Keyboard> {
    Box::new(keyboard)
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
            score: 1.,
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
            score: 2.,
            version: "version2".to_string(),
        };

        assert_eq!(&k1, &k2);

        let vec = [k1, k2].iter().cloned().unique().collect_vec();

        assert_eq!(vec.len(), 1);
    }
}
