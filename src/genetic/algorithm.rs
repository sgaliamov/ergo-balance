// use itertools::Itertools;
// use rayon::prelude::*;
// use std::cmp::Ordering;

// use crate::{Behaviour, Individual};

// pub struct GeneticAlgorithm {
//     behaviour: Box<dyn Behaviour>,
// }

// impl GeneticAlgorithm {
//     pub fn new(behaviour: Box<dyn Behaviour>) -> Self {
//         GeneticAlgorithm { behaviour }
//     }

//     pub fn run(
//         &self,
//         population: &mut Vec<Box<dyn Individual>>,
//     ) -> Result<Vec<Box<dyn Individual>>, ()> {
//         let context = self.behaviour.get_context();
//         let mut mutants: Vec<_> = population
//             .into_par_iter()
//             .flat_map(|parent| {
//                 (0..context.children_count)
//                     .map(|_| self.behaviour.mutate(parent))
//                     .collect::<Vec<_>>()
//             })
//             .collect::<Vec<_>>();

//         mutants.append(population);

//         let offspring: Vec<_> = mutants
//             .into_iter()
//             .unique()
//             .sorted_by(GeneticAlgorithm::score_cmp)
//             .group_by(|x| x.parent_version.clone())
//             .into_iter()
//             .map(|(_, group)| group.collect())
//             .collect::<Vec<_>>()
//             .into_par_iter()
//             .flat_map(|group| self.recombine(group))
//             .collect::<Vec<Box<dyn Individual>>>()
//             .into_iter()
//             .unique()
//             .sorted_by(GeneticAlgorithm::score_cmp)
//             .into_iter()
//             .take(context.population_size)
//             .collect();

//         if offspring.len() == 0 {
//             return Err(());
//         }

//         Ok(offspring)
//     }

//     fn score_cmp(&self, a: &dyn Individual, b: &dyn Individual) -> Ordering {
//         let a_total = self.behaviour.get_score(a);
//         let b_total = self.behaviour.get_score(b);

//         b_total.partial_cmp(&a_total).unwrap()
//     }

//     fn recombine(&self, collection: Vec<Box<dyn Individual>>) -> Vec<Box<dyn Individual>> {
//         if collection.len() == 1 {
//             return collection;
//         }

//         let mut crossed = collection
//             .iter()
//             .tuple_windows()
//             .map(|(a, b)| self.behaviour.cross(&a, &b.get_mutations()))
//             .collect_vec();

//         crossed.extend(collection);

//         crossed
//     }
// }
