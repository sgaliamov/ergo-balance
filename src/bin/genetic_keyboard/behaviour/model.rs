use ed_balance::Context;
use std::collections::{HashMap, HashSet};

pub struct Behaviour {
    pub context: Context,
    pub words: Vec<String>,

    /// char * position
    pub frozen_keys: FrozenKeys,

    /// Includes positions of frozen keys.
    pub blocked_keys: HashSet<u8>,
    pub efforts: Efforts,
    pub switch_penalty: f64,
    pub same_key_penalty: f64,
}

pub type Efforts = HashMap<u8, HashMap<u8, f64>>;

pub type FrozenKeys = HashMap<char, u8>;
