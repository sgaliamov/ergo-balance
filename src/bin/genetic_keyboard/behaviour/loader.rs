use super::{Behaviour, Efforts, FrozenKeys};
use ed_balance::{CliSettings, Context};
use serde_json::{self, Value};
use std::{collections::HashMap, path::PathBuf};

pub fn create(settings: &CliSettings) -> Option<Behaviour> {
    let context = Context::new(settings);
    let json = load_json(&settings.keyboard)?;
    let sample_path = json["samplePath"].as_str()?;
    let sample_text = std::fs::read_to_string(sample_path).ok()?;
    let frozen_keys = load_frozen(&json)?;
    let efforts = load_efforts(&json)?;

    Some(Behaviour {
        context,
        sample_text,
        frozen_keys,
        efforts,
    })
}

fn parse_u8(str: &String) -> Option<u8> {
    str.parse::<u8>().ok()
}

// todo: test
fn normalize_effort(value: f64, factor: f64) -> f64 {
    (value - 1.) * factor + 1.
}

fn parse_nested(json: &Value, factor: f64) -> Option<HashMap<u8, f64>> {
    json.as_object()?
        .iter()
        .map(|(key, value)| {
            let key = parse_u8(key)?;
            let value = normalize_effort(value.as_f64()?, factor);
            Some((key, value))
        })
        .collect()
}

fn load_efforts(json: &Value) -> Option<Efforts> {
    let max = json["maxEffort"].as_f64()?;
    let factor = max / 5.;

    json["efforts"]
        .as_object()?
        .iter()
        .map(|(key, value)| {
            let key = parse_u8(key)?;
            let value = parse_nested(value, factor)?;
            Some((key, value))
        })
        .collect()
}

fn load_json(keyboard: &PathBuf) -> Option<Value> {
    let content = std::fs::read_to_string(keyboard).ok()?;
    serde_json::from_str(&content).ok()
}

fn load_frozen(json: &Value) -> Option<FrozenKeys> {
    json["frozen"]
        .as_object()?
        .iter()
        .map(|(key, value)| {
            let key = parse_u8(key)?;
            let value = value.as_str()?.chars().next()?;
            Some((key, value))
        })
        .collect()
}
