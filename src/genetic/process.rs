use crate::{CliSettings, DynError, GeneticAlgorithm, IBehaviour, IIndividual, IMutation};
use chrono::prelude::*;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use itertools::Itertools;
use std::{sync::Arc, thread};

pub fn run<TMutation, TIndividual, TBehaviour>(settings: CliSettings) -> Result<(), DynError>
where
    TMutation: IMutation,
    TIndividual: IIndividual<TMutation>,
    TBehaviour: IBehaviour<TMutation, TIndividual>,
{
    let settings = Arc::new(settings);
    let progress = MultiProgress::new();
    let pb_main = ProgressBar::new(settings.generations_count as u64);
    pb_main.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar} {pos}/{len} ({eta}) {msg}"),
    );
    let pb_main = progress.add(pb_main);
    let spinner_style = ProgressStyle::default_spinner().template("{wide_msg}");
    let progress_bars: Vec<ProgressBar> = (0..settings.results_count)
        .map(|_| {
            let pb = ProgressBar::new_spinner();
            pb.set_style(spinner_style.clone());
            progress.add(pb)
        })
        .collect();

    let settings = Arc::clone(&settings);
    let _ = thread::spawn(move || {
        let behaviour = TBehaviour::new(&settings);
        let context = behaviour.get_context();
        let algorithm = GeneticAlgorithm::new(&behaviour);

        let mut population: Vec<_> = (0..context.population_size)
            .into_iter()
            .map(|_| behaviour.generate())
            .collect();
        population.extend(behaviour.load().unwrap());

        // to be able just calculate scores
        if context.generations_count == 0 {
            TBehaviour::save(&population).unwrap();
            return;
        }

        let mut prev: DateTime<Utc> = Utc::now();
        let mut prev_top_result = Vec::<Box<TIndividual>>::new();
        let mut repeats_counter = 0;

        for index in 0..context.generations_count {
            population = algorithm.run(&population, &context).expect("All died!");

            let (repeats, top_results, to_continue) = need_to_continue(
                repeats_counter,
                &prev_top_result,
                &population,
                context.results_count,
                context.repeats_count,
                &behaviour,
            );

            prev_top_result = top_results;
            repeats_counter = repeats;

            if !to_continue {
                pb_main.set_message(&format!("(repeats: {})", repeats_counter + 1));
                break;
            }

            if let Some(date) = render_progress::<_, _, TBehaviour>(
                index,
                prev,
                &pb_main,
                &progress_bars,
                &prev_top_result,
                context.generations_count,
                repeats_counter,
            ) {
                prev = date;
            }
        }

        render_progress::<_, _, TBehaviour>(
            context.generations_count - 1,
            prev,
            &pb_main,
            &progress_bars,
            &prev_top_result,
            context.generations_count,
            repeats_counter,
        );
        pb_main.finish();
        progress_bars.iter().for_each(|x| x.finish());
    });

    progress.join()?;

    Ok(())
}

fn need_to_continue<TMutation, TIndividual, TBehaviour>(
    mut repeats_counter: u16,
    prev_result: &Vec<Box<TIndividual>>,
    population: &Vec<Box<TIndividual>>,
    results_count: usize,
    max_repeats_count: u16,
    behaviour: &TBehaviour,
) -> (u16, Vec<Box<TIndividual>>, bool)
where
    TIndividual: IIndividual<TMutation>,
    TMutation: IMutation,
    TBehaviour: IBehaviour<TMutation, TIndividual>,
{
    let top_results = population
        .iter()
        .take(results_count)
        .sorted_by(|&a, &b| {
            behaviour
                .score_cmp(a, b)
                .then_with(|| a.to_string().cmp(&b.to_string()))
        })
        .cloned()
        .collect_vec();

    if prev_result.eq(&top_results) {
        repeats_counter += 1;
    } else {
        repeats_counter = 0;
    }

    if repeats_counter == max_repeats_count {
        return (repeats_counter, top_results, false);
    }

    (repeats_counter, top_results, true)
}

fn render_progress<TMutation, TIndividual, TBehaviour>(
    index: u16,
    prev: DateTime<Utc>,
    pb_main: &ProgressBar,
    progress_bars: &Vec<ProgressBar>,
    results: &Vec<Box<TIndividual>>,
    generations_count: u16,
    repeats_counter: u16,
) -> Option<DateTime<Utc>>
where
    TIndividual: IIndividual<TMutation>,
    TMutation: IMutation,
    TBehaviour: IBehaviour<TMutation, TIndividual>,
{
    let passed = Utc::now() - prev;

    if passed.num_seconds() >= 5 || index == 0 || index == generations_count - 1 {
        pb_main.set_message(&format!("(repeats: {})", repeats_counter));

        for (i, item) in results.iter().enumerate() {
            let text = item.to_string();
            progress_bars[i].set_message(&text);
        }

        pb_main.set_position(index as u64);
        TBehaviour::save(&results).unwrap();

        return Some(Utc::now());
    }

    None
}
