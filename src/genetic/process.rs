use crate::{CliSettings, DynError, GeneticAlgorithm, IBehaviour, IIndividual, IMutation};
use chrono::prelude::*;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
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
        let mut prev_result = Vec::<Box<TIndividual>>::new();
        let mut repeats_counter = 0;

        for index in 0..context.generations_count {
            population = algorithm.run(&mut population).expect("All died!");

            if let Some(date) = render_progress(
                index,
                prev,
                &pb_main,
                &progress_bars,
                &population,
                context.generations_count,
            ) {
                prev = date
            }

            if let Some((repeats, top_results)) = need_to_continue(
                repeats_counter,
                &prev_result,
                &population,
                context.results_count,
                context.repeats_count,
            ) {
                prev_result = top_results;
                repeats_counter = repeats;
                pb_main.set_message(&format!("(repeats: {})", repeats_counter));
            } else {
                pb_main.set_message(&format!("(repeats: {})", repeats_counter + 1));
                break;
            }

            TBehaviour::save(&prev_result).unwrap();
        }

        pb_main.finish();
        progress_bars.iter().for_each(|x| x.finish());
    });

    progress.join()?;

    Ok(())
}

fn need_to_continue<TMutation, TIndividual>(
    mut repeats: u16,
    prev_result: &Vec<Box<TIndividual>>,
    population: &Vec<Box<TIndividual>>,
    results_count: usize,
    repeats_count: u16,
) -> Option<(u16, Vec<Box<TIndividual>>)>
where
    TIndividual: IIndividual<TMutation>,
    TMutation: IMutation,
{
    let top_results: Vec<_> = population
        .iter()
        .take(results_count)
        .map(|x| x.clone())
        .collect();

    if prev_result.eq(&top_results) {
        repeats += 1;
    } else {
        repeats = 0;
    }

    if repeats == repeats_count {
        return None;
    }

    Some((repeats, top_results))
}

fn render_progress<TMutation, TIndividual>(
    index: u16,
    prev: DateTime<Utc>,
    pb_main: &ProgressBar,
    progress_bars: &Vec<ProgressBar>,
    population: &Vec<Box<TIndividual>>,
    generations_count: u16,
) -> Option<DateTime<Utc>>
where
    TIndividual: IIndividual<TMutation>,
    TMutation: IMutation,
{
    let passed = Utc::now() - prev;

    if passed.num_seconds() >= 5 || index == 0 || index == generations_count - 1 {
        for (i, item) in population.iter().take(progress_bars.len()).enumerate() {
            let text = item.to_string();
            progress_bars[i].set_message(&text);
        }

        pb_main.set_position(index as u64);

        return Some(Utc::now());
    }

    None
}
