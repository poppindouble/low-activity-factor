use std::env;
use wellen::*;

const LOAD_OPTS: LoadOptions = LoadOptions {
    multi_thread: true,
    remove_scopes_with_empty_name: false,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config_name = &args[1];
    let benchmark = &args[2];
    let dir = "./waveforms/".to_string() + config_name + "/" + benchmark;
    println!(
        "Design name {}, benchmark name {}",
        config_name.to_string(),
        benchmark.to_string()
    );

    let header = viewers::read_header_from_file(dir, &LOAD_OPTS).expect("Failed to load file!");

    let hierarchy = header.hierarchy;
    let body = viewers::read_body(header.body, &hierarchy, None).expect("Failed to load body!");

    let capacity = body.time_table.len();
    let mut activity_factor = (0..capacity).map(|_| 0).collect::<Vec<_>>();
    println!("total cycles {:#?}", capacity);

    let mut wave_source = body.source;

    let ids = hierarchy
        .get_unique_signals_vars()
        .iter()
        .flatten()
        .map(|var| var.signal_ref())
        .collect::<Vec<_>>();

    // Total number of signals
    let ids_len = ids.len();

    let loaded = wave_source.load_signals(&ids, &hierarchy, LOAD_OPTS.multi_thread);

    // _loaded_id changes _signal_change at time time
    for (_loaded_id, loaded_signal) in loaded {
        for (time, _signal_change) in loaded_signal.iter_changes() {
            // The total number of signal changes at time time
            activity_factor[time as usize] += 1;
        }
    }

    let activity_factor = activity_factor
        .into_iter()
        .map(|v| (v as f32) / (ids_len as f32))
        .collect::<Vec<_>>();

    println!("Total number of signals {:#?}", ids_len);

    fn average(xs: &[f32]) -> f32 {
        let sum: f32 = xs.iter().sum();
        sum / (xs.len() as f32)
    }

    println!(
        "Average activity factor {:#?}%",
        average(&activity_factor) * 100.0
    );
    println!(
        "Max activity factor {:#?}",
        activity_factor
            .iter()
            .max_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap()
    );
    println!(
        "Min activity factor {:#?}",
        activity_factor
            .iter()
            .min_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap()
    );

    fn get_percent(activity_factor: &[f32], percent: u32) -> f32 {
        let total = activity_factor
            .iter()
            .filter(|v| **v < (percent as f32 / 100.0))
            .collect::<Vec<_>>()
            .len();
        total as f32 / activity_factor.len() as f32
    }

    println!(
        "Percent of activity factors lower then 20% {:#?}%",
        get_percent(&activity_factor, 20) * 100.0
    );
    println!(
        "Percent of activity factors lower then 10% {:#?}%",
        get_percent(&activity_factor, 10) * 100.0
    );
    println!(
        "Percent of activity factors lower then 5% {:#?}%",
        get_percent(&activity_factor, 5) * 100.0
    );
}
