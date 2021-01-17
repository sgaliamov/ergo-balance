use ed_balance::Context;
use std::collections::HashMap;

pub struct Behaviour {
    pub context: Context,
    pub sample_text: String,
    pub frozen_keys: HashMap<u8, char>,
    pub efforts: HashMap<u8, HashMap<u8, i32>>,
}
