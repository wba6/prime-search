mod algorithms;
mod export;
mod progress;
mod stats;

use clap::{Arg, Command};

use crate::algorithms::{
    atkin::run_sieve_of_atkin, eratosthenes::run_sieve_of_eratosthenes,
    miller_rabin::run_miller_rabin, trial_division::run_trial_division,
};
use std::time::Duration;

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
                .help("Algorithm to run (eratosthenes, trial, atkin, miller-rabin)"),
        )
        .arg(
            Arg::new("max")
                .short('m')
                .long("max")
                .action(clap::ArgAction::SetTrue)
                .help("Run the algorithm continuously to find the largest prime possible"),
        )
        .arg(
            Arg::new("time")
                .short('t')
                .long("time")
                .num_args(1)
                .value_parser(clap::value_parser!(f64))
                .help("Specify the time in seconds for which the algorithm should run"),
        )
        .get_matches();

    let algorithm = matches.get_one::<String>("algorithm").map(|s| s.to_lowercase());

    let run_until_stopped = matches.get_flag("max");
    let time_limit = matches.get_one::<f64>("time").copied();

    // Debug print to verify time_limit value
    println!("Time limit: {:?}", time_limit);

    if let Some(limit_seconds) = time_limit {
        if limit_seconds <= 0.0 {
            eprintln!("Error: Time limit must be greater than zero.");
            return;
        }
    }

    if algorithm.is_none() && time_limit.is_none() {
        eprintln!("Error: Either specify an algorithm using -a or a time limit using -t.");
        return;
    }

    if algorithm.is_none() && time_limit.is_some() {
        // Run all algorithms for the specified time
        let limit_seconds = time_limit.unwrap();

        println!("Running all algorithms for {:.2} seconds each.", limit_seconds);

        run_sieve_of_eratosthenes(run_until_stopped, Some(limit_seconds));
        run_trial_division(run_until_stopped, Some(limit_seconds));
        run_sieve_of_atkin(run_until_stopped, Some(limit_seconds));
        run_miller_rabin(run_until_stopped, Some(limit_seconds));
    } else if let Some(alg) = algorithm {
        match alg.as_str() {
            "eratosthenes" => {
                run_sieve_of_eratosthenes(run_until_stopped, time_limit);
            }
            "trial" => {
                run_trial_division(run_until_stopped, time_limit);
            }
            "atkin" => {
                run_sieve_of_atkin(run_until_stopped, time_limit);
            }
            "miller-rabin" => {
                run_miller_rabin(run_until_stopped, time_limit);
            }
            _ => {
                eprintln!("Unknown algorithm: {}", alg);
                return;
            }
        }
    } else {
        eprintln!("Error: You must specify an algorithm using -a if no time limit is provided.");
    }
}
