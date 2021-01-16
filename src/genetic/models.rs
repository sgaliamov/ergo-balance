use std::{collections::HashSet, hash::Hash};

use crate::CliSettings;

pub trait IIndividual<TMutation>: Clone + Eq + Hash + Send + Sync + PartialEq
where
    TMutation: IMutation,
{
    fn get_mutations<'a>(&'a self) -> &'a Vec<TMutation>;
    fn get_parent_version(&self) -> String;
    fn to_string(&self) -> String;
}

pub trait IMutation: Sync {}

pub trait IBehaviour<TMutation, TIndividual>: Sync
where
    TMutation: IMutation,
    TIndividual: IIndividual<TMutation>,
{
    fn new(settings: &CliSettings) -> Self;

    fn generate(&self) -> Box<TIndividual>;

    fn get_score(&self, individual: &TIndividual) -> f64;

    fn cross(
        &self,
        individual: &TIndividual,
        partner_mutations: &Vec<TMutation>,
    ) -> Box<TIndividual>;

    fn mutate(&self, individual: &TIndividual) -> Box<TIndividual>;

    fn get_context<'a>(&'a self) -> &'a Context;
}

pub struct Context {
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

impl Context {
    pub fn new(settings: &CliSettings) -> Self {
        let mut frozen_left = HashSet::with_capacity(settings.frozen_left.len());
        frozen_left.extend(settings.frozen_left.chars());

        let mut frozen_right = HashSet::with_capacity(settings.frozen_right.len());
        frozen_right.extend(settings.frozen_right.chars());

        Context {
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
