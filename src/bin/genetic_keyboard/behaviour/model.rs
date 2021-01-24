use ed_balance::Context;
use std::collections::{HashMap, HashSet};

pub type Position = i32;

pub struct Behaviour {
    pub context: Context,
    pub words: Vec<String>,

    /// char * position
    pub frozen_keys: FrozenKeys,

    /// Includes positions of frozen keys.
    pub blocked_keys: HashSet<Position>,
    pub efforts: Efforts,
    pub switch_penalty: f64,
    pub same_key_penalty: f64,
}

pub type Efforts = HashMap<Position, HashMap<Position, f64>>;

pub type FrozenKeys = HashMap<char, Position>;

// impl Behaviour {
//     pub fn new(
//         context: Context,
//         words: Vec<String>,
//         frozen_keys: FrozenKeys,
//         blocked_keys: HashSet<Position>,
//         efforts: Efforts,
//         switch_penalty: f64,
//         same_key_penalty: f64,
//     ) -> Self {
//         Behaviour {
//             context,
//             words,
//             frozen_keys,
//             blocked_keys,
//             efforts,
//             switch_penalty,
//             same_key_penalty,
//         }
//     }
// }
