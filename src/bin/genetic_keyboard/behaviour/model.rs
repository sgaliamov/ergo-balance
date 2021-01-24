use ed_balance::Context;
use std::collections::HashMap;

pub struct Behaviour {
    pub context: Context,
    pub words: Vec<String>,
    pub frozen_keys: FrozenKeys,
    pub efforts: Efforts,
    pub switch_penalty: f64,
}

pub type Efforts = HashMap<u8, HashMap<u8, f64>>;

pub type FrozenKeys = HashMap<u8, char>;
