use ed_balance::{IIndividual, IMutation};
use itertools::{sorted, Itertools};
use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Clone, Copy)]
pub struct Mutation {
    from: usize,
    to: usize,
}

impl IMutation for Mutation {}

#[derive(Debug, Clone)]
pub struct Keyboard {
    pub version: String,

    /// A list of 30 characters.\
    /// Each position represents a position on a keyboard.\
    /// 0-14 the left part, 15-29 the right part.\
    /// The numbering like in the `ergo-layout` app.\
    /// `_` means a skipped and blocked key.
    pub keys: HashMap<char, u8>,
    pub score: f64,

    pub mutations: Vec<Mutation>,
    pub parent_version: String,
    pub parent: HashMap<char, u8>,
}

impl Eq for Keyboard {}

impl PartialEq for Keyboard {
    fn eq(&self, other: &Self) -> bool {
        self.keys.iter().eq(other.keys.iter())
    }
}

impl Hash for Keyboard {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for item in &self.keys {
            item.hash(state);
        }
    }
}

impl IIndividual<Mutation> for Keyboard {
    fn get_kind(&self) -> String {
        self.parent_version.clone()
    }

    fn to_string(&self) -> String {
        let keys_string: String = self
            .keys
            .iter()
            .sorted_by(|(_, i1), (_, i2)| i1.partial_cmp(i2).unwrap())
            .map(|(c, _)| c)
            .collect();

        format!("{}; {:.3};", keys_string, self.score)
    }
}
