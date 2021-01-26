use ed_balance::Context;
use std::collections::{HashMap, HashSet};

pub type Position = u8;

pub struct Behaviour {
    pub context: Context,
    pub words: Vec<String>,

    /// char * position
    pub frozen_keys: FrozenKeys,

    /// Does not include positions of frozen keys.
    pub blocked_keys: HashSet<Position>,
    pub efforts: Efforts,
    pub switch_penalty: f64,
    pub same_key_penalty: f64,
}

pub type Efforts = HashMap<Position, HashMap<Position, f64>>;

pub type FrozenKeys = HashMap<char, Position>;
