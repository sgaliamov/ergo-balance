use super::letters::{Letters, LettersCollection};
use ed_balance::models::{Context, Individual};
use itertools::Itertools;
use rayon::prelude::*;

pub fn run(population: &mut LettersCollection, context: &Context) -> Result<LettersCollection, ()> {
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
        .sorted_by(Letters::score_cmp)
        .group_by(|x| x.parent_version.clone())
        .into_iter()
        .map(|(_, group)| group.collect())
        .collect::<Vec<_>>()
        .into_par_iter()
        .flat_map(|group| recombine(group, context))
        .collect::<LettersCollection>()
        .into_iter()
        .unique()
        .sorted_by(Letters::score_cmp)
        .into_iter()
        .take(context.population_size)
        .collect();

    if offspring.len() == 0 {
        return Err(());
    }

    Ok(offspring)
}

fn recombine(collection: LettersCollection, context: &Context) -> LettersCollection {
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
