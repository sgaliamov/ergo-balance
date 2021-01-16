use ed_balance::{format_result, Digraphs, IIndividual, IMutation};
use std::hash::Hash;

pub type LettersPointer = Box<Letters>;

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

    fn get_parent_version(&self) -> String {
        self.parent_version.clone()
    }

    fn to_string(&self) -> String {
        format_result(&self.left, &self.right, self.left_score, self.right_score)
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
}

fn box_letters(letters: Letters) -> LettersPointer {
    Box::new(letters)
}
