use std::hash::Hash;

use crate::CliSettings;

pub trait IIndividual<TMutation: IMutation>: Clone + Eq + Hash + Send + Sync {
    /// The kind of individual.
    fn get_kind(&self) -> String;

    /// Text representation.
    fn to_string(&self) -> String;
}

pub trait IMutation: Sync {}

pub trait IBehaviour<TMutation: IMutation, TIndividual: IIndividual<TMutation>>: Sync {
    fn new(settings: &CliSettings) -> Self;

    fn generate(&self) -> Box<TIndividual>;

    fn get_score(&self, individual: &TIndividual) -> f64;

    fn cross(&self, individual: &TIndividual, partner: &TIndividual) -> Box<TIndividual>;

    fn mutate(&self, individual: &TIndividual) -> Box<TIndividual>;

    fn get_context<'a>(&'a self) -> &'a Context;
}

pub struct Context {
    pub mutations_count: usize,
    pub population_size: usize,
    pub children_count: u16,
    pub generations_count: u16,
    pub results_count: usize,
    pub left_count: usize,
    pub repeats_count: u16,
}

impl Context {
    pub fn new(settings: &CliSettings) -> Self {
        Context {
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
