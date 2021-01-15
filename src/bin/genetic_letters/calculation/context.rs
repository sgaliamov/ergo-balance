use ed_balance::models::{CliSettings, Digraphs};
use std::collections::HashSet;

pub struct LettersContext {
    pub digraphs: Digraphs,
    pub frozen_left: HashSet<char>,
    pub frozen_right: HashSet<char>,
    pub mutations_count: usize,
    pub population_size: usize,
    pub children_count: u16,
    pub generations_count: u16,
    pub results_count: usize,
    pub left_count: usize,
    pub repeats_count: u16,
}

impl LettersContext {
    pub fn new(settings: &CliSettings) -> Self {
        let digraphs = Digraphs::load(&settings.digraphs).unwrap();

        let mut frozen_left = HashSet::with_capacity(settings.frozen_left.len());
        frozen_left.extend(settings.frozen_left.chars());

        let mut frozen_right = HashSet::with_capacity(settings.frozen_right.len());
        frozen_right.extend(settings.frozen_right.chars());

        LettersContext {
            digraphs,
            frozen_left,
            frozen_right,
            mutations_count: settings.mutations_count as usize,
            population_size: settings.population_size as usize,
            children_count: settings.children_count,
            generations_count: settings.generations_count,
            results_count: settings.results_count as usize,
            left_count: settings.left_count as usize,
            repeats_count: settings.repeats_count,
        }
    }
}