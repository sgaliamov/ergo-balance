use crate::{Context, IBehaviour, IIndividual, IMutation};
use itertools::Itertools;
use rand::{thread_rng, RngCore};
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

    pub fn run(
        &self,
        population: &Vec<Box<TIndividual>>,
        context: &Context,
    ) -> Result<Vec<Box<TIndividual>>, ()> {
        let best_population_size = context.population_size / 2;

        let sorted = population
            .iter()
            .sorted_by(|a, b| self.behaviour.score_cmp(a, b))
            .cloned()
            .collect_vec();

        let mut best = sorted
            .iter()
            .take(best_population_size)
            .cloned()
            .collect_vec();

        let best = self.process(&mut best, best_population_size, context.children_count)?;

        let mut rest = sorted
            .iter()
            .skip(best_population_size)
            .cloned()
            .collect_vec();

        let mut rest = self.process(&mut rest, context.population_size, context.children_count)?;

        let top_rest = rest[0].get_score();

        let mut best = best
            .into_iter()
            .filter(|x| x.get_score() < top_rest)
            .collect_vec();
        best.append(&mut rest);

        Ok(best)
    }

    fn process(
        &self,
        population: &mut Vec<Box<TIndividual>>,
        population_size: usize,
        children_count: u32,
    ) -> Result<Vec<Box<TIndividual>>, ()> {
        let mut rng = thread_rng();
        let max_children_count = 1 + (rng.next_u32() % children_count);
        let mut mutants: Vec<_> = population
            .into_par_iter()
            .flat_map(|parent| {
                (0..max_children_count)
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
            .take(population_size)
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
