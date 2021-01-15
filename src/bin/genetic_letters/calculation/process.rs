use super::{
    context::LettersContext,
    letters::{LettersCollection, LettersPointer},
};
use itertools::Itertools;
use rayon::prelude::*;
use std::cmp::Ordering;

pub struct GeneticAlgorithm<'a> {
    context: &'a LettersContext,
}

impl<'a> GeneticAlgorithm<'a> {
    pub fn new(context: &'a LettersContext) -> Self {
        GeneticAlgorithm { context }
    }

    pub fn run(
        &self,
        population: &mut LettersCollection,
        context: &LettersContext,
    ) -> Result<LettersCollection, ()> {
        let mut mutants: Vec<_> = population
            .into_par_iter()
            .flat_map(|parent| {
                (0..context.children_count)
                    .map(|_| parent.mutate(context))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        mutants.append(population);

        let offspring: Vec<_> = mutants
            .into_iter()
            .unique()
            .sorted_by(GeneticAlgorithm::score_cmp)
            .group_by(|x| x.parent_version.clone())
            .into_iter()
            .map(|(_, group)| group.collect())
            .collect::<Vec<_>>()
            .into_par_iter()
            .flat_map(|group| self.recombine(group, context))
            .collect::<LettersCollection>()
            .into_iter()
            .unique()
            .sorted_by(GeneticAlgorithm::score_cmp)
            .into_iter()
            .take(context.population_size)
            .collect();

        if offspring.len() == 0 {
            return Err(());
        }

        Ok(offspring)
    }

    fn score_cmp(a: &LettersPointer, b: &LettersPointer) -> Ordering {
        let a_total = a.get_score();
        let b_total = b.get_score();

        b_total.partial_cmp(&a_total).unwrap()
    }

    fn recombine(
        &self,
        collection: LettersCollection,
        context: &LettersContext,
    ) -> LettersCollection {
        if collection.len() == 1 {
            return collection;
        }

        let mut crossed = collection
            .iter()
            .tuple_windows()
            .map(|(a, b)| a.cross(&b.mutations, context))
            .collect_vec();

        crossed.extend(collection);

        crossed
    }
}
