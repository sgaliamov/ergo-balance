use super::LettersBehaviour;
use ed_balance::{Digraphs, IIndividual, IMutation, get_version};
use itertools::Itertools;
use rand::prelude::SliceRandom;
use std::hash::Hash;

pub type LettersPointer = Box<Letters>;

pub type LettersCollection = Vec<LettersPointer>;

#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Clone, Copy)]
pub struct Mutation {
    pub left: char,
    pub right: char,
}

impl IMutation for Mutation {}

#[derive(Debug, Clone)]
pub struct Letters {
    pub version: String,
    pub left: Vec<char>,
    pub right: Vec<char>,
    pub left_score: f64,
    pub right_score: f64,
    pub mutations: Vec<Mutation>,
    pub parent_version: String,
    pub parent_left: Vec<char>,
    pub parent_right: Vec<char>,
}

impl IIndividual<Mutation> for Letters {
    fn get_mutations(&self) -> &Vec<Mutation> {
        &self.mutations
    }
}

impl Eq for Letters {}

impl PartialEq for Letters {
    fn eq(&self, other: &Letters) -> bool {
        self.left.eq(&other.left) && self.right.eq(&other.right)
    }
}

impl Hash for Letters {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.left.hash(state);
        self.right.hash(state);
    }
}

impl Letters {
    pub fn ctor(
        version: String,
        left: &Vec<char>,
        right: &Vec<char>,
        mutations: Vec<Mutation>,
        parent_version: String,
        parent_left: Vec<char>,
        parent_right: Vec<char>,
        digraphs: &Digraphs,
    ) -> LettersPointer {
        let mut sorted_left = left.clone();
        let mut sorted_right = right.clone();
        sorted_left.sort();
        sorted_right.sort();

        let left_score = digraphs.calculate_score(&sorted_left);
        let right_score = digraphs.calculate_score(&sorted_right);

        box_letters(Letters {
            left: sorted_left,
            right: sorted_right,
            left_score,
            right_score,
            version,
            mutations,
            parent_version,
            parent_left,
            parent_right,
        })
    }

    pub fn copy(&self) -> LettersPointer {
        box_letters(Letters {
            left: self.left.clone(),
            right: self.right.clone(),
            left_score: self.left_score,
            right_score: self.right_score,
            version: self.version.clone(),
            mutations: self.mutations.clone(),
            parent_version: self.parent_version.clone(),
            parent_left: self.parent_left.clone(),
            parent_right: self.parent_right.clone(),
        })
    }

    pub fn new(behaviour: &LettersBehaviour) -> LettersPointer {
        let context = &behaviour.context;
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

        Self::ctor(
            version.clone(),
            &left,
            &right,
            Vec::new(),
            version, // versions match to be able cross children with parents
            left.clone(),
            right.clone(),
            &behaviour.digraphs,
        )
    }
}

fn box_letters(letters: Letters) -> LettersPointer {
    Box::new(letters)
}
