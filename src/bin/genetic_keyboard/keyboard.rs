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
    pub keys: Vec<char>,
    pub score: f64,

    pub mutations: Vec<Mutation>,
    pub parent_version: String,
    pub parent: Vec<char>,
}

impl Eq for Keyboard {}

impl PartialEq for Keyboard {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Hash for Keyboard {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        todo!()
    }
}

impl IIndividual<Mutation> for Keyboard {
    fn get_kind(&self) -> String {
        todo!()
    }

    fn to_string(&self) -> String {
        todo!()
    }
}
