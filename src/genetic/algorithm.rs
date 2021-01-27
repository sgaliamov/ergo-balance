use crate::{IBehaviour, IIndividual, IMutation};
use itertools::Itertools;
use rayon::prelude::*;
use std::marker::PhantomData;

pub struct GeneticAlgorithm<'a, TMutation, TIndividual, TBehaviour>
where
    TMutation: IMutation,
    TIndividual: IIndividual<TMutation>,
    TBehaviour: IBehaviour<TMutation, TIndividual>,
{
    behaviour: &'a TBehaviour,
    phantom_mutation: PhantomData<TMutation>, // todo: find a better way
    phantom_individual: PhantomData<TIndividual>,
}

impl<'a, TMutation, TIndividual, TBehaviour>
    GeneticAlgorithm<'a, TMutation, TIndividual, TBehaviour>
where
    TMutation: IMutation,
    TIndividual: IIndividual<TMutation>,
    TBehaviour: IBehaviour<TMutation, TIndividual>,
{
    pub fn new(behaviour: &'a TBehaviour) -> Self {
        GeneticAlgorithm {
            behaviour,
            phantom_mutation: PhantomData,
            phantom_individual: PhantomData,
        }
    }

    pub fn run(&self, population: &mut Vec<Box<TIndividual>>) -> Result<Vec<Box<TIndividual>>, ()> {
        let context = self.behaviour.get_context();
        let mut mutants: Vec<_> = population
            .into_par_iter()
            .flat_map(|parent| {
                (0..context.children_count)
                    .map(|_| self.behaviour.mutate(parent))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        mutants.append(population);

        let offspring: Vec<_> = mutants
            .into_iter()
            .unique()
            .sorted_by(|a, b| self.behaviour.score_cmp(a, b))
            .group_by(|x| x.get_kind())
            .into_iter()
            .map(|(_, group)| group.collect())
            .collect::<Vec<_>>()
            .into_par_iter()
            .flat_map(|group| self.recombine(group))
            .collect::<Vec<_>>()
            .into_iter()
            .unique()
            .sorted_by(|a, b| self.behaviour.score_cmp(a, b))
            .into_iter()
            .take(context.population_size)
            .collect();

        if offspring.len() == 0 {
            return Err(());
        }

        Ok(offspring)
    }

    fn recombine(&self, collection: Vec<Box<TIndividual>>) -> Vec<Box<TIndividual>> {
        if collection.len() == 1 {
            return collection;
        }

        let mut crossed = collection
            .iter()
            .tuple_windows()
            .map(|(a, b)| self.behaviour.cross(&a, &b))
            .collect_vec();

        crossed.extend(collection);
        crossed
    }
}
