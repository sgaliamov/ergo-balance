mod digraphs;
mod shared;

use std::cmp::Ordering;

pub use digraphs::*;
pub use shared::*;

pub trait Individual<T> {
    fn score_cmp(a: &Box<T>, b: &Box<T>) -> Ordering;
}

// pub trait Context
// {

// }
