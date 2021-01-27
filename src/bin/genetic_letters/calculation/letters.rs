use ed_balance::{format_result, Digraphs, IIndividual, IMutation};
use std::hash::Hash;

pub type LettersPointer = Box<Letters>;

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub struct Mutation {
    pub left: char,
    pub right: char,
}

impl IMutation for Mutation {}

#[derive(Clone)]
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
    fn get_kind(&self) -> String {
        self.parent_version.clone()
    }

    fn to_string(&self) -> String {
        format_result(&self.left, &self.right, self.left_score, self.right_score)
    }
}

impl Letters {
    pub fn new(
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

fn box_letters(letters: Letters) -> LettersPointer {
    Box::new(letters)
}
