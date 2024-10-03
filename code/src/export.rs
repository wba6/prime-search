use crate::stats::AlgorithmStats;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::Path;

pub fn export_to_csv(stat: &AlgorithmStats, filename: &str) {
    let file_exists = Path::new(filename).exists();
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
        .expect("Could not open file");
    let mut writer = BufWriter::new(file);

    if !file_exists {
        // Write header if file did not exist
        writeln!(
            writer,
            "Algorithm,Time Taken (s),Largest Prime Found,Number of Primes Found"
        )
            .unwrap();
    }

    writeln!(
        writer,
        "{},{:.6},{},{}",
        stat.algorithm_name,
        stat.time_taken.as_secs_f64(),
        stat.largest_prime,
        stat.num_primes_found
    )
        .unwrap();
}
