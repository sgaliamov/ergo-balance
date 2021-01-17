use ed_balance::{IIndividual, IMutation};
use std::hash::Hash;

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
    pub keys: Vec<char>,
    pub score: f64,

    pub mutations: Vec<Mutation>,
    pub parent_version: String,
    pub parent: Vec<char>,
}

impl Eq for Keyboard {}

impl PartialEq for Keyboard {
    fn eq(&self, other: &Self) -> bool {
        self.keys.eq(&other.keys)
    }
}

impl Hash for Keyboard {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.keys.hash(state)
    }
}

impl IIndividual<Mutation> for Keyboard {
    fn get_kind(&self) -> String {
        self.parent_version.clone()
    }

    fn to_string(&self) -> String {
        let keys_string: String = self.keys.iter().collect();
        format!("{}; {:.3};", keys_string, self.score)
    }
}
