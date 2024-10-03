mod algorithms;
mod export;
mod progress;
mod stats;

use clap::{Arg, Command};

use crate::algorithms::{
    atkin::run_sieve_of_atkin, eratosthenes::run_sieve_of_eratosthenes,
    miller_rabin::run_miller_rabin, trial_division::run_trial_division,
};

fn main() {
    // Parse command-line arguments
    let matches = Command::new("Prime Finder")
        .version("1.0")
        .author("Your Name <youremail@example.com>")
        .about("Finds primes using various algorithms")
        .arg(
            Arg::new("algorithm")
                .short('a')
                .long("algorithm")
                .num_args(1)
                .required(true)
                .help("Algorithm to run (eratosthenes, trial, atkin, miller-rabin)"),
        )
        .arg(
            Arg::new("max")
                .short('m')
                .long("max")
                .action(clap::ArgAction::SetTrue)
                .help("Run the algorithm continuously to find the largest prime possible"),
        )
        .get_matches();

    let algorithm = matches
        .get_one::<String>("algorithm")
        .expect("Algorithm is required");

    let run_until_stopped = matches.get_flag("max");

    match algorithm.as_str() {
        "eratosthenes" => {
            run_sieve_of_eratosthenes(run_until_stopped);
        }
        "trial" => {
            run_trial_division(run_until_stopped);
        }
        "atkin" => {
            run_sieve_of_atkin(run_until_stopped);
        }
        "miller-rabin" => {
            run_miller_rabin(run_until_stopped);
        }
        _ => {
            eprintln!("Unknown algorithm: {}", algorithm);
            return;
        }
    }
}
