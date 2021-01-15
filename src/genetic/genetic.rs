// use itertools::Itertools;
// use rayon::prelude::*;
// use std::{cmp::Ordering, collections::HashSet};

// pub trait Individual<T> {
//     fn score_cmp(a: &Box<T>, b: &Box<T>) -> Ordering;
// }

// pub trait Context {
//       fn  get_frozen_left()->  HashSet<char>;
//       fn  get_frozen_right()->  HashSet<char>;
//       fn  get_mutations_count()->  usize;
//       fn  get_population_size()->  usize;
//       fn  get_children_count()->  u16;
//       fn  get_generations_count()->  u16;
//       fn  get_results_count()->  usize;
//       fn  get_left_count()->  usize;
//       fn  get_repeats_count()->  u16;
// }

// pub struct GeneticAlgorithm {
//     context: Context;

//     pub fn new(context: Context)-> GeneticAlgorithm{
//         GeneticAlgorithm{
//             context
//         }
//     }

//     // pub fn run(population: &mut LettersCollection) -> Result<LettersCollection, ()> {
//     //     let mut mutants: Vec<_> = population
//     //         .into_par_iter()
//     //         .flat_map(|parent| {
//     //             (0..context.children_count)
//     //                 .map(|_| parent.mutate(context))
//     //                 .collect::<Vec<_>>()
//     //         })
//     //         .collect::<Vec<_>>();

//     //     mutants.append(population);

//     //     let offspring: Vec<_> = mutants
//     //         .into_iter()
//     //         .unique()
//     //         .sorted_by(Letters::score_cmp)
//     //         .group_by(|x| x.parent_version.clone())
//     //         .into_iter()
//     //         .map(|(_, group)| group.collect())
//     //         .collect::<Vec<_>>()
//     //         .into_par_iter()
//     //         .flat_map(|group| recombine(group, context))
//     //         .collect::<LettersCollection>()
//     //         .into_iter()
//     //         .unique()
//     //         .sorted_by(Letters::score_cmp)
//     //         .into_iter()
//     //         .take(context.population_size)
//     //         .collect();

//     //     if offspring.len() == 0 {
//     //         return Err(());
//     //     }

//     //     Ok(offspring)
//     // }

//     // fn recombine(collection: LettersCollection, context: &Context) -> LettersCollection {
//     //     if collection.len() == 1 {
//     //         return collection;
//     //     }

//     //     let mut crossed = collection
//     //         .iter()
//     //         .tuple_windows()
//     //         .map(|(a, b)| a.cross(&b.mutations, context))
//     //         .collect_vec();

//     //     crossed.extend(collection);

//     //     crossed
//     // }

// }

