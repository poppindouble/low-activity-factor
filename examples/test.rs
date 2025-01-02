use wellen::*;

const WAVEFORM_FILE: &str = "./waveforms/multiply.vcd";

const LOAD_OPTS: LoadOptions = LoadOptions {
    multi_thread: true,
    remove_scopes_with_empty_name: false,
};

fn main() {
    let header =
        viewers::read_header_from_file(WAVEFORM_FILE, &LOAD_OPTS).expect("Failed to load file!");

    let hierarchy = header.hierarchy;
    let body = viewers::read_body(header.body, &hierarchy, None).expect("Failed to load body!");

    let capacity = body.time_table.len();
    let mut activity_factor = (0..capacity).map(|_| 0).collect::<Vec<_>>();

    let mut wave_source = body.source;

    let ids = hierarchy
        .get_unique_signals_vars()
        .iter()
        .flatten()
        .map(|var| var.signal_ref())
        .collect::<Vec<_>>();
    let ids_len = ids.len();

    let loaded = wave_source.load_signals(&ids, &hierarchy, LOAD_OPTS.multi_thread);

    for (_loaded_id, loaded_signal) in loaded {
        for (time, _signal_change) in loaded_signal.iter_changes() {
            activity_factor[time as usize] += 1;
        }
    }

    let activity_factor = activity_factor
        .into_iter()
        .map(|v| (v as f32) / (ids_len as f32))
        .collect::<Vec<_>>();

    let total = activity_factor.len();
    let under_20_percent = activity_factor
        .iter()
        .filter(|v| **v < 0.2)
        .collect::<Vec<_>>()
        .len();
    let under_10_percent = activity_factor
        .iter()
        .filter(|v| **v < 0.1)
        .collect::<Vec<_>>()
        .len();
    let under_1_percent = activity_factor
        .iter()
        .filter(|v| **v < 0.01)
        .collect::<Vec<_>>()
        .len();

    println!("{activity_factor:#?}");
    println!("{total:#?}");
    println!("{under_20_percent:#?}");
    println!("{under_10_percent:#?}");
    println!("{under_1_percent:#?}");
}
