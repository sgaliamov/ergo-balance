use super::{Behaviour, Efforts, FrozenKeys, Position};
use ed_balance::{CliSettings, Context};
use itertools::Itertools;
use serde_json::{self, Value};
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

pub fn create(settings: &CliSettings) -> Option<Behaviour> {
    let context = Context::new(settings);
    let path = settings.keyboard.clone()?;
    let json = load_json(&path)?;
    let words = load_words(&settings.text.clone()?)?;
    let frozen_keys = load_frozen(&json)?;
    let efforts = load_efforts(&json)?;
    let switch_penalty = json["switchPenalty"].as_f64()?;
    let same_key_penalty = json["sameKeyPenalty"].as_f64()?;
    let blocked_keys: HashSet<Position> = json["blocked"]
        .as_array()?
        .into_iter()
        .map(|x| x.as_u64().unwrap() as Position)
        .collect();

    Some(Behaviour {
        context,
        words,
        frozen_keys,
        efforts,
        switch_penalty,
        same_key_penalty,
        blocked_keys,
    })
}

fn load_words(path: &PathBuf) -> Option<Vec<String>> {
    let text = std::fs::read_to_string(path).ok()?;
    let words = text.split(' ').map_into().collect_vec();
    Some(words)
}

fn parse_u8(str: &String) -> Option<Position> {
    str.parse::<Position>().ok()
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

fn parse_nested_efforts(
    json: &Value,
    keys_shift: Position,
    factor: f64,
) -> Option<HashMap<Position, f64>> {
    json.as_object()?
        .iter()
        .map(|(key, value)| {
            let key = parse_u8(key)? + keys_shift;
            let value = normalize_effort(value.as_f64()?, factor);
            Some((key, value))
        })
        .collect()
}

fn parse_efforts(json: &Value, keys_shift: Position, factor: f64) -> Option<Efforts> {
    json["efforts"]
        .as_object()?
        .iter()
        .map(|(key, value)| {
            let key = parse_u8(key)? + keys_shift;
            let value = parse_nested_efforts(value, keys_shift, factor)?;
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
    // the right part is symmetrical to the left so we can just add 15 to get right efforts.
    // for a standard keyboard it will be easier to have all efforts in the json file.
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
            let key = key.chars().next()?;
            let value = parse_u8(&value.as_str()?.to_string())?;
            Some((key, value))
        })
        .collect()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_load() {
        let json = json!({
        "maxEffort": 5,
        "efforts": {
            "0": {
                "0": 1,
                "1": 2
            },
            "1": {
                "0": 3,
                "1": 4
            },
        }});
        let actual = load_efforts(&json).unwrap();
        let expected: Efforts = [
            (0, [(0, 1.), (1, 2.)].iter().cloned().collect()),
            (1, [(0, 3.), (1, 4.)].iter().cloned().collect()),
            (15, [(15, 1.), (16, 2.)].iter().cloned().collect()),
            (16, [(15, 3.), (16, 4.)].iter().cloned().collect()),
        ]
        .iter()
        .cloned()
        .collect();

        assert_eq!(actual, expected);
    }

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
