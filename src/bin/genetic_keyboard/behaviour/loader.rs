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
    let switch_penalty = json["switchPenalty"].as_f64()?;

    Some(Behaviour {
        context,
        sample_text,
        frozen_keys,
        efforts,
        switch_penalty,
    })
}

fn parse_u8(str: &String) -> Option<u8> {
    str.parse::<u8>().ok()
}

const MIN_VALUE: f64 = 1.;
const MAX_VALUE: f64 = 5.;

fn normalize_effort(value: f64, factor: f64) -> f64 {
    debug_assert!(
        value >= MIN_VALUE,
        format!("Minimal allowed value is {}", MIN_VALUE)
    );
    debug_assert!(
        value <= 5.,
        format!("Maximal allowed value is {}", MAX_VALUE)
    );

    (value - 1.) * factor + 1.
}

fn parse_nested_efforts(json: &Value, factor: f64) -> Option<HashMap<u8, f64>> {
    json.as_object()?
        .iter()
        .map(|(key, value)| {
            let key = parse_u8(key)?;
            let value = normalize_effort(value.as_f64()?, factor);
            Some((key, value))
        })
        .collect()
}

fn parse_efforts(json: &Value, keys_shift: u8, factor: f64) -> Option<Efforts> {
    json["efforts"]
        .as_object()?
        .iter()
        .map(|(key, value)| {
            let key = parse_u8(key)? + keys_shift;
            let value = parse_nested_efforts(value, factor)?;
            Some((key, value))
        })
        .collect()
}

fn get_factor(max: f64) -> f64 {
    (max - 1.) / (MAX_VALUE - 1.)
}

fn load_efforts(json: &Value) -> Option<Efforts> {
    let max = json["maxEffort"].as_f64()?;
    let factor = get_factor(max);

    let mut left = parse_efforts(json, 0, factor)?;
    let right = parse_efforts(json, 15, factor)?;
    left.extend(right);

    Some(left)
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

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_normalize_effort_for_1() {
        let factor = get_factor(3.);
        let actual = normalize_effort(1., factor);

        assert_eq!(actual, 1.);
    }

    #[test]
    fn test_normalize_effort_for_2() {
        let factor = get_factor(2.);
        let actual = normalize_effort(3., factor);

        assert_eq!(actual, 1.5);
    }

    #[test]
    fn test_normalize_effort_for_3() {
        let factor = get_factor(3.);
        let actual = normalize_effort(3., factor);

        assert_eq!(actual, 2.);
    }

    #[test]
    fn test_normalize_effort_for_4() {
        let factor = get_factor(4.);
        let actual = normalize_effort(3., factor);

        assert_eq!(actual, 2.5);
    }

    #[test]
    fn test_normalize_effort_for_5() {
        let factor = get_factor(3.);
        let actual = normalize_effort(5., factor);

        assert_eq!(actual, 3.);
    }
}
