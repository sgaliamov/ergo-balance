use super::{Letters, LettersPointer, Mutation};
use ed_balance::{
    get_score, get_version,
    models::{CliSettings, Digraphs},
};
use ed_balance::{Context, IBehaviour};
use itertools::Itertools;
use rand::prelude::SliceRandom;
use std::collections::HashSet;

pub struct LettersBehaviour {
    pub context: Context,
    pub digraphs: Digraphs,
}

impl LettersBehaviour {
    pub fn new(settings: &CliSettings) -> Self {
        let digraphs = Digraphs::load(&settings.digraphs).unwrap();
        let context = Context::new(settings);

        LettersBehaviour { digraphs, context }
    }

    pub fn create(&self) -> LettersPointer {
        let context = &self.context;
        let mut all = ('a'..='z')
            .filter(|&x| !context.frozen_right.contains(&x))
            .filter(|&x| !context.frozen_left.contains(&x))
            .collect_vec();

        all.shuffle(&mut rand::thread_rng());

        let mut left = context.frozen_left.iter().map(|&x| x).collect_vec();
        left.append(
            &mut all
                .iter()
                .take(context.left_count - left.len())
                .map(|&x| x)
                .collect(),
        );

        let mut right = context.frozen_right.iter().map(|&x| x).collect_vec();
        right.append(
            &mut all
                .iter()
                .filter(|x| !left.contains(x))
                .map(|&x| x)
                .collect(),
        );

        let version = get_version();

        Letters::new(
            version.clone(),
            &left,
            &right,
            Vec::new(),
            version, // versions match to be able cross children with parents
            left.clone(),
            right.clone(),
            &self.digraphs,
        )
    }
}

impl IBehaviour<Mutation, Letters> for LettersBehaviour {
    fn get_score(&self, individual: &Letters) -> f64 {
        get_score(individual.left_score, individual.right_score)
    }

    fn cross(&self, individual: &Letters, partner_mutations: &Vec<Mutation>) -> LettersPointer {
        let mut left = individual.parent_left.clone();
        let mut right = individual.parent_right.clone();
        let mut mutations: Vec<_> = individual
            .mutations
            .iter()
            .chain(partner_mutations.iter())
            .unique()
            .map(|&x| x)
            .collect();

        mutations.shuffle(&mut rand::thread_rng());

        for mutation in mutations.iter().take(self.context.mutations_count) {
            let left_index = left.iter().position(|&x| x == mutation.left);
            let right_index = right.iter().position(|&x| x == mutation.right);

            match (left_index, right_index) {
                (Some(left_index), Some(right_index)) => {
                    left[left_index] = mutation.left;
                    right[right_index] = mutation.right;
                }
                _ => panic!("Incompatible mutation!"),
            }
        }

        Letters::new(
            get_version(),
            &left,
            &right,
            mutations, // this mutations is not just a sum of 2 mutations, it's an intersection.
            individual.parent_version.clone(), // so, to be able to get the current state,
            individual.parent_left.clone(), // we have apply this mutations on the initial parent letters.
            individual.parent_right.clone(), // current - mutations = parent.
            &self.digraphs,
        )
    }

    // fn mutate(&self) -> LettersPointer {
    //     let mut rng = thread_rng();

    //     let mut left = self
    //         .left
    //         .iter()
    //         .filter(|&x| !context.frozen_left.contains(x))
    //         .map(|&x| x)
    //         .collect_vec();
    //     left.shuffle(&mut rng);

    //     let mut right = self
    //         .right
    //         .iter()
    //         .filter(|&x| !context.frozen_right.contains(x))
    //         .map(|&x| x)
    //         .collect_vec();
    //     right.shuffle(&mut rng);

    //     let mut mutations: Vec<_> = Vec::with_capacity(context.mutations_count);

    //     let mutations_count = min(vec![context.mutations_count, left.len(), right.len()]).unwrap();

    //     for index in 0..mutations_count {
    //         let left_char = left[index];
    //         let right_char = right[index];

    //         left[index] = right_char;
    //         right[index] = left_char;

    //         mutations.push(Mutation {
    //             left: left_char,
    //             right: right_char,
    //         });
    //     }

    //     left.extend(&context.frozen_left.iter().map(|&x| x).collect_vec());
    //     right.extend(&context.frozen_right.iter().map(|&x| x).collect_vec());

    //     Self::ctor(
    //         get_version(),
    //         &left,
    //         &right,
    //         mutations,
    //         self.version.clone(),
    //         self.left.clone(),
    //         self.right.clone(),
    //         &context.digraphs,
    //     )
    // }

    fn get_context<'a>(&'a self) -> &'a Context {
        &self.context
    }
}

// #[cfg(test)]
// pub mod tests {
//     use super::*;
//     use serde_json::json;
//     use std::collections::HashSet;

//     fn default_context() -> Context {
//         Context {
//             frozen_left: HashSet::new(),
//             frozen_right: HashSet::new(),
//             mutations_count: 4,
//             population_size: 10,
//             children_count: 10,
//             generations_count: 10,
//             results_count: 10,
//             left_count: 15,
//             repeats_count: 10,
//         }
//     }

//     #[test]
//     fn unique_should_work() {
//         let json = json!({});
//         let digraphs = Digraphs::new(&json.as_object().unwrap());
//         let context = default_context();
//         let behaviour = LettersBehaviour { digraphs, context };
//         let a = Letters::new(&behaviour);
//         let b = Letters::new(&behaviour);
//         let clone = a.clone();
//         let vec: LettersCollection = vec![a, b, clone];

//         let actual: LettersCollection = vec.into_iter().unique().collect();

//         assert_eq!(actual.len(), 2);
//     }

//     #[test]
//     fn should_assign_parent_version() {
//         let json = json!({});
//         let digraphs = Digraphs::new(&json.as_object().unwrap());
//         let mut context = default_context();
//         let behaviour = LettersBehaviour { digraphs, context };
//         context.mutations_count = 1;

//         let target = Letters::new(&behaviour);
//         let actual = behaviour.mutate(&target);

//         assert_eq!(actual.parent_version, target.version);
//     }

//     #[test]
//     fn should_not_mutate_source_object() {
//         let json = json!({});
//         let digraphs = Digraphs::new(&json.as_object().unwrap());
//         let context = default_context(digraphs);
//         let target = Letters::new(&context);
//         let copy = target.left.clone();
//         let actual = target.mutate(&context);

//         assert_ne!(actual.left, copy);
//         assert_eq!(copy, target.left);
//     }

//     #[test]
//     fn should_mutate() {
//         let json = json!({});
//         let digraphs = Digraphs::new(&json.as_object().unwrap());
//         let context = default_context(digraphs);
//         let target = Letters::new(&context);

//         let actual = target.mutate(&context);

//         assert_ne!(target.left, actual.left);
//         assert_ne!(target.right, actual.right);
//     }

//     #[test]
//     fn should_sort_chars() {
//         let json = json!({});
//         let digraphs = Digraphs::new(&json.as_object().unwrap());
//         let context = default_context(digraphs);
//         let letters = Letters::new(&context);

//         let target = to_sorted_string(&letters.left);
//         let actual: String = letters.left.iter().collect();

//         assert_eq!(target, actual);

//         let target = to_sorted_string(&letters.right);
//         let actual: String = letters.right.iter().collect();

//         assert_eq!(target, actual);
//     }

//     fn to_sorted_string(list: &Vec<char>) -> String {
//         let mut vec = list.clone();
//         vec.sort();
//         vec.iter().collect()
//     }
// }
