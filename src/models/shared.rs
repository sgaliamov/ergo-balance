use rand::{distributions::Alphanumeric, Rng};
use std::{cmp::Ordering, error::Error, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct CliSettings {
    /// keyboard settings
    #[structopt(short = "k", long = "keyboard")]
    pub keyboard: Option<PathBuf>,

    /// sample text
    #[structopt(short = "t", long = "text")]
    pub text: Option<PathBuf>,

    #[structopt(short = "d", long = "digraphs")]
    pub digraphs: Option<PathBuf>,

    #[structopt(long = "frozen-left", default_value = "")]
    pub frozen_left: String,

    #[structopt(long = "frozen-right", default_value = "")]
    pub frozen_right: String,

    #[structopt(short = "m", long = "mutations-count", default_value = "2")]
    pub mutations_count: u8,

    #[structopt(short = "p", long = "population-size", default_value = "100")]
    pub population_size: u16,

    #[structopt(short = "c", long = "children-count", default_value = "10")]
    pub children_count: u16,

    /// how long we run a genetic algorithm.
    #[structopt(short = "g", long = "generations-count", default_value = "1000")]
    pub generations_count: u16,

    /// how much we render in at the end
    #[structopt(short = "r", long = "results-count", default_value = "20")]
    pub results_count: u8,

    #[structopt(short = "l", long = "left-count", default_value = "15")]
    pub left_count: u8,

    /// how much we continue on the same result.\
    /// if generations are not evolving not much sense to continue.
    #[structopt(long = "repeats-count", default_value = "100")]
    pub repeats_count: u16,
}

pub type DynError = Box<dyn Error>;

pub fn get_version() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(|x| x.to_string())
        .collect()
}

pub fn print_letters(
    left_letters: &Vec<char>,
    right_letters: &Vec<char>,
    left_score: f64,
    right_score: f64,
) {
    println!(
        "{}",
        format_result(left_letters, right_letters, left_score, right_score)
    );
}

pub fn format_result(
    left_letters: &Vec<char>,
    right_letters: &Vec<char>,
    left_score: f64,
    right_score: f64,
) -> String {
    let left_string: String = left_letters.iter().collect();
    let right_string: String = right_letters.iter().collect();

    format!(
        "{}; {}; {:.3}; {}; {}; {:.3}; {:.3}; {:.3}; {:.3};",
        left_letters.len(),
        left_string,
        left_score,
        right_letters.len(),
        right_string,
        right_score,
        get_factor(left_score, right_score),
        left_score + right_score,
        get_score(left_score, right_score)
    )
}

fn get_factor(left_score: f64, right_score: f64) -> f64 {
    let factor = if left_score.partial_cmp(&right_score).unwrap() == Ordering::Less {
        left_score / right_score
    } else {
        right_score / left_score
    };

    1.1 - 0.1 / factor
}

pub fn get_score(left: f64, right: f64) -> f64 {
    let factor = get_factor(left, right);
    let total = left + right;

    total * factor * factor * factor
}
