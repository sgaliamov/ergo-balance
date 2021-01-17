use super::Behaviour;
use ed_balance::CliSettings;

pub fn behaviour_new(settings: &CliSettings) -> Behaviour {
    // let context = Context::new(settings);

    // let content = std::fs::read_to_string(&settings.keyboard).unwrap();
    // let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    // let rawData = json.as_object().unwrap();

    // let map = json
    //     .iter()
    //     .map(|(digraph, value)| {
    //         let first = digraph.chars().nth(0)?;
    //         let second = digraph.chars().nth(1)?;
    //         let third = value.as_f64()?;
    //         Some((first, second, third))
    //     })
    //     .map(|some| some.unwrap())
    //     .fold(DigraphsMap::new(), |mut result, (first, second, value)| {
    //         result
    //             .entry(first)
    //             .or_insert(HashMap::new())
    //             .insert(second, value);
    //         result
    //     });

    // Behaviour { context }
    todo!()
}
