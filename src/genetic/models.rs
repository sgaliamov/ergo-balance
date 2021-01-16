use std::collections::HashSet;

use crate::CliSettings;

pub trait IIndividual<TMutation>
where
    TMutation: IMutation,
{
    fn get_mutations<'a>(&'a self) -> &'a Vec<TMutation>;
}

pub trait IMutation {}

pub trait IBehaviour<TMutation, TIndividual>
where
    TMutation: IMutation,
    TIndividual: IIndividual<TMutation>,
{
    fn get_score(&self, individual: &TIndividual) -> f64;

    fn cross(
        &self,
        individual: &TIndividual,
        partner_mutations: &Vec<TMutation>,
    ) -> Box<TIndividual>;

    // fn mutate(&self, individual: &Box<dyn Individual>) -> Box<dyn Individual>;

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

// pub type BehaviourPointer<TMutation, TIndividual>
// where
//     TMutation: Mutation,
//     TIndividual: Individual,
// = Box<dyn Behaviour<TMutation, TIndividual>>;

// pub trait Context {
//     fn get_frozen_left(&self) -> HashSet<char>;
//     fn get_frozen_right(&self) -> HashSet<char>;
//     fn get_mutations_count(&self) -> usize;
//     fn get_population_size(&self) -> usize;
//     fn get_children_count(&self) -> u16;
//     fn get_generations_count(&self) -> u16;
//     fn get_results_count(&self) -> usize;
//     fn get_left_count(&self) -> usize;
//     fn get_repeats_count(&self) -> u16;
// }
