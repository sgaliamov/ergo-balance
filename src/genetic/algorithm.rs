use itertools::Itertools;
use rayon::prelude::*;
use std::{cmp::Ordering, marker::PhantomData};

use crate::{IBehaviour, IIndividual, IMutation};

pub struct GeneticAlgorithm<TMutation, TIndividual, TBehaviour>
where
    TMutation: IMutation,
    TIndividual: IIndividual<TMutation>,
    TBehaviour: IBehaviour<TMutation, TIndividual>,
{
    behaviour: TBehaviour,
    phantom_mutation: PhantomData<TMutation>, // todo: find better way
    phantom_individual: PhantomData<TIndividual>,
}

impl<TMutation, TIndividual, TBehaviour> GeneticAlgorithm<TMutation, TIndividual, TBehaviour>
where
    TMutation: IMutation,
    TIndividual: IIndividual<TMutation>,
    TBehaviour: IBehaviour<TMutation, TIndividual>,
{
    pub fn new(behaviour: TBehaviour) -> Self {
        GeneticAlgorithm {
            behaviour,
            phantom_mutation: PhantomData,
            phantom_individual: PhantomData,
        }
    }

    // pub fn run(
    //     &self,
    //     population: &mut Vec<Box<dyn Individual>>,
    // ) -> Result<Vec<Box<dyn Individual>>, ()> {
    //     let context = self.behaviour.get_context();
    //     let mut mutants: Vec<_> = population
    //         .into_par_iter()
    //         .flat_map(|parent| {
    //             (0..context.children_count)
    //                 .map(|_| self.behaviour.mutate(parent))
    //                 .collect::<Vec<_>>()
    //         })
    //         .collect::<Vec<_>>();

    //     mutants.append(population);

    //     let offspring: Vec<_> = mutants
    //         .into_iter()
    //         .unique()
    //         .sorted_by(GeneticAlgorithm::score_cmp)
    //         .group_by(|x| x.parent_version.clone())
    //         .into_iter()
    //         .map(|(_, group)| group.collect())
    //         .collect::<Vec<_>>()
    //         .into_par_iter()
    //         .flat_map(|group| self.recombine(group))
    //         .collect::<Vec<Box<dyn Individual>>>()
    //         .into_iter()
    //         .unique()
    //         .sorted_by(GeneticAlgorithm::score_cmp)
    //         .into_iter()
    //         .take(context.population_size)
    //         .collect();

    //     if offspring.len() == 0 {
    //         return Err(());
    //     }

    //     Ok(offspring)
    // }

    fn score_cmp(&self, a: &TIndividual, b: &TIndividual) -> Ordering {
        let a_total = self.behaviour.get_score(a);
        let b_total = self.behaviour.get_score(b);

        b_total.partial_cmp(&a_total).unwrap()
    }

    fn recombine(&self, collection: Vec<Box<TIndividual>>) -> Vec<Box<TIndividual>> {
        if collection.len() == 1 {
            return collection;
        }

        let mut crossed = collection
            .iter()
            .tuple_windows()
            .map(|(a, b)| self.behaviour.cross(&a, &b.get_mutations()))
            .collect_vec();

        crossed.extend(collection);

        crossed
    }
}
