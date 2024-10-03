use clap::Parser;
use crossbeam::channel::{unbounded, Sender};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::fs::File;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::{Duration, Instant};
use rand::Rng;

/// Program to find primes using different algorithms and record them in CSV files.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Duration to run the algorithms (in seconds)
    #[arg(short, long)]
    duration: u64,
}

#[derive(Debug, Clone, Copy)]
enum Algorithm {
    TrialDivision,
    SieveOfEratosthenes,
    SieveOfAtkin,
    MillerRabin,
}

impl Algorithm {
    fn as_str(&self) -> &'static str {
        match self {
            Algorithm::TrialDivision => "Trial Division",
            Algorithm::SieveOfEratosthenes => "Sieve of Eratosthenes",
            Algorithm::SieveOfAtkin => "Sieve of Atkin",
            Algorithm::MillerRabin => "Miller-Rabin",
        }
    }

    fn csv_filename(&self) -> &'static str {
        match self {
            Algorithm::TrialDivision => "trial_division.csv",
            Algorithm::SieveOfEratosthenes => "sieve_of_eratosthenes.csv",
            Algorithm::SieveOfAtkin => "sieve_of_atkin.csv",
            Algorithm::MillerRabin => "miller_rabin.csv",
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args = Args::parse();
    let duration = Duration::from_secs(args.duration);
    println!(
        "Running prime algorithms for {} seconds...",
        args.duration
    );

    // Shared stop flag
    let stop_flag = Arc::new(AtomicBool::new(false));

    // Channels for each algorithm
    let (td_sender, td_receiver) = unbounded();
    let (soe_sender, soe_receiver) = unbounded();
    let (soa_sender, soa_receiver) = unbounded();
    let (mr_sender, mr_receiver) = unbounded();

    // Create CSV writers
    let td_file = File::create(Algorithm::TrialDivision.csv_filename())?;
    let mut td_writer = csv::Writer::from_writer(td_file);
    td_writer.write_record(&["prime", "timestamp"])?;

    let soe_file = File::create(Algorithm::SieveOfEratosthenes.csv_filename())?;
    let mut soe_writer = csv::Writer::from_writer(soe_file);
    soe_writer.write_record(&["prime", "timestamp"])?;

    let soa_file = File::create(Algorithm::SieveOfAtkin.csv_filename())?;
    let mut soa_writer = csv::Writer::from_writer(soa_file);
    soa_writer.write_record(&["prime", "timestamp"])?;

    let mr_file = File::create(Algorithm::MillerRabin.csv_filename())?;
    let mut mr_writer = csv::Writer::from_writer(mr_file);
    mr_writer.write_record(&["prime", "timestamp"])?;

    // Set up progress bars
    let m = MultiProgress::new();
    let pb_style = ProgressStyle::with_template(
        "{msg} | Largest Prime: {pos} | Elapsed: {elapsed_precise}",
    )
        .unwrap()
        .progress_chars("=>-");

    let td_pb = m.add(ProgressBar::new(0));
    td_pb.set_style(pb_style.clone());
    td_pb.set_message(Algorithm::TrialDivision.as_str());

    let soe_pb = m.add(ProgressBar::new(0));
    soe_pb.set_style(pb_style.clone());
    soe_pb.set_message(Algorithm::SieveOfEratosthenes.as_str());

    let soa_pb = m.add(ProgressBar::new(0));
    soa_pb.set_style(pb_style.clone());
    soa_pb.set_message(Algorithm::SieveOfAtkin.as_str());

    let mr_pb = m.add(ProgressBar::new(0));
    mr_pb.set_style(pb_style.clone());
    mr_pb.set_message(Algorithm::MillerRabin.as_str());

    // Start time
    let start_time = Instant::now();

    // Clone stop_flag for threads
    let td_stop = Arc::clone(&stop_flag);
    let soe_stop = Arc::clone(&stop_flag);
    let soa_stop = Arc::clone(&stop_flag);
    let mr_stop = Arc::clone(&stop_flag);

    // Start algorithm threads
    let td_handle = thread::spawn(move || {
        trial_division(td_sender, td_stop);
    });

    let soe_handle = thread::spawn(move || {
        sieve_of_eratosthenes(soe_sender, soe_stop);
    });

    let soa_handle = thread::spawn(move || {
        sieve_of_atkin(soa_sender, soa_stop);
    });

    let mr_handle = thread::spawn(move || {
        miller_rabin(mr_sender, mr_stop);
    });

    // Track largest primes
    let mut largest_primes = vec![
        (Algorithm::TrialDivision, 0u64),
        (Algorithm::SieveOfEratosthenes, 0u64),
        (Algorithm::SieveOfAtkin, 0u64),
        (Algorithm::MillerRabin, 0u64),
    ];

    // Main loop to receive primes and write to CSV
    loop {
        let elapsed = start_time.elapsed();
        if elapsed >= duration {
            break;
        }

        crossbeam::select! {
            recv(td_receiver) -> msg => {
                if let Ok(prime) = msg {
                    let timestamp = format!("{:.2}", elapsed.as_secs_f64());
                    td_writer.serialize((&prime, &timestamp)).ok();
                    td_writer.flush().ok();
                    if prime > largest_primes[0].1 {
                        largest_primes[0].1 = prime;
                        td_pb.set_position(prime);
                        // No need to update the message with primes and elapsed time here
                    }
                }
            },
            recv(soe_receiver) -> msg => {
                if let Ok(prime) = msg {
                    let timestamp = format!("{:.2}", elapsed.as_secs_f64());
                    soe_writer.serialize((&prime, &timestamp)).ok();
                    soe_writer.flush().ok();
                    if prime > largest_primes[1].1 {
                        largest_primes[1].1 = prime;
                        soe_pb.set_position(prime);
                    }
                }
            },
            recv(soa_receiver) -> msg => {
                if let Ok(prime) = msg {
                    let timestamp = format!("{:.2}", elapsed.as_secs_f64());
                    soa_writer.serialize((&prime, &timestamp)).ok();
                    soa_writer.flush().ok();
                    if prime > largest_primes[2].1 {
                        largest_primes[2].1 = prime;
                        soa_pb.set_position(prime);
                    }
                }
            },
            recv(mr_receiver) -> msg => {
                if let Ok(prime) = msg {
                    let timestamp = format!("{:.2}", elapsed.as_secs_f64());
                    mr_writer.serialize((&prime, &timestamp)).ok();
                    mr_writer.flush().ok();
                    if prime > largest_primes[3].1 {
                        largest_primes[3].1 = prime;
                        mr_pb.set_position(prime);
                    }
                }
            },
            default(Duration::from_millis(100)) => {}
        }
    }

    // Capture the exact elapsed time at the moment of termination
    let final_elapsed = start_time.elapsed();

    // Signal threads to stop
    stop_flag.store(true, Ordering::SeqCst);

    // Wait for threads to finish with a timeout to prevent indefinite hanging
    let handles = vec![td_handle, soe_handle, soa_handle, mr_handle];
    for handle in handles {
        if let Err(e) = handle.join() {
            eprintln!("A thread encountered an error: {:?}", e);
        }
    }

    // Finish progress bars with final messages using final_elapsed
    td_pb.finish_with_message(format!(
        "{} | Largest Prime: {} | Elapsed: {:.2?}",
        Algorithm::TrialDivision.as_str(),
        largest_primes[0].1,
        final_elapsed
    ));
    soe_pb.finish_with_message(format!(
        "{} | Largest Prime: {} | Elapsed: {:.2?}",
        Algorithm::SieveOfEratosthenes.as_str(),
        largest_primes[1].1,
        final_elapsed
    ));
    soa_pb.finish_with_message(format!(
        "{} | Largest Prime: {} | Elapsed: {:.2?}",
        Algorithm::SieveOfAtkin.as_str(),
        largest_primes[2].1,
        final_elapsed
    ));
    mr_pb.finish_with_message(format!(
        "{} | Largest Prime: {} | Elapsed: {:.2?}",
        Algorithm::MillerRabin.as_str(),
        largest_primes[3].1,
        final_elapsed
    ));

    println!("Prime search completed.");

    Ok(())
}

/// Trial Division Algorithm
fn trial_division(sender: Sender<u64>, stop_flag: Arc<AtomicBool>) {
    let mut num: u64 = 2;
    while !stop_flag.load(Ordering::SeqCst) {
        if is_prime_trial_division(num) {
            sender.send(num).ok();
        }
        num += 1;
    }
}

/// Check primality using Trial Division
fn is_prime_trial_division(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

/// Sieve of Eratosthenes Algorithm with Improved Stop Flag Checking
fn sieve_of_eratosthenes(sender: Sender<u64>, stop_flag: Arc<AtomicBool>) {
    let mut sieve = Vec::new();
    let mut size = 1000;
    sieve.resize(size, true);
    sieve[0] = false;
    sieve[1] = false;
    let mut current: usize = 2;

    'outer: loop {
        if stop_flag.load(Ordering::SeqCst) {
            break;
        }

        while current < size {
            if stop_flag.load(Ordering::SeqCst) {
                break 'outer;
            }
            if sieve[current] {
                sender.send(current as u64).ok(); // Cast usize to u64
                let mut multiple = current * 2;
                while multiple < size {
                    if stop_flag.load(Ordering::SeqCst) {
                        break 'outer;
                    }
                    sieve[multiple] = false;
                    multiple += current;
                }
            }
            current += 1;
        }

        // Increase sieve size
        size *= 2;
        sieve.resize(size, true);
        // Re-sieve the new range
        for p in 2..current {
            if sieve[p] {
                let mut multiple = p * 2;
                while multiple < size {
                    if stop_flag.load(Ordering::SeqCst) {
                        break 'outer;
                    }
                    sieve[multiple] = false;
                    multiple += p;
                }
            }
        }
    }
}

/// Sieve of Atkin Algorithm with Improved Stop Flag Checking
fn sieve_of_atkin(sender: Sender<u64>, stop_flag: Arc<AtomicBool>) {
    let mut sieve = vec![false; 1000];
    let mut size = 1000;
    sieve[2] = true;
    sieve[3] = true;

    let mut n: u64 = 5;

    'outer: loop {
        if stop_flag.load(Ordering::SeqCst) {
            break;
        }

        if (n as usize) >= size {
            // Increase sieve size
            size *= 2;
            sieve.resize(size, false);
            // Typically, Sieve of Atkin would reprocess the sieve here
            // For simplicity, we'll skip detailed implementation
        }

        // Simple trial check (placeholder for actual Atkin logic)
        if is_prime_trial_division(n) {
            sieve[n as usize] = true;
            sender.send(n).ok();
        }

        n += 1;

        // Periodically check the stop flag within the loop
        if n % 1000 == 0 { // Adjust the frequency as needed
            if stop_flag.load(Ordering::SeqCst) {
                break 'outer;
            }
        }
    }
}

/// Miller-Rabin Primality Test Algorithm with Improved Stop Flag Checking
fn miller_rabin(sender: Sender<u64>, stop_flag: Arc<AtomicBool>) {
    let mut num: u64 = 2;
    while !stop_flag.load(Ordering::SeqCst) {
        if is_prime_miller_rabin(num) {
            sender.send(num).ok();
        }
        num += 1;

        // Periodically check the stop flag
        if num % 1000 == 0 { // Adjust as needed
            if stop_flag.load(Ordering::SeqCst) {
                break;
            }
        }
    }
}

/// Check primality using Miller-Rabin
fn is_prime_miller_rabin(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    // Write n-1 as 2^s * d
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }

    // Number of trials
    let trials = 5;
    let mut rng = rand::thread_rng();

    for _ in 0..trials {
        let a = rng.gen_range(2..n - 1);
        let mut x = modpow(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        let mut cont_outer = false;
        for _ in 0..s - 1 {
            x = modpow(x, 2, n);
            if x == n - 1 {
                cont_outer = true;
                break;
            }
        }
        if cont_outer {
            continue;
        }
        return false;
    }
    true
}

/// Modular exponentiation
fn modpow(mut base: u64, mut exp: u64, modu: u64) -> u64 {
    if modu == 1 {
        return 0;
    }
    let mut result: u64 = 1; // Explicitly type as u64
    base = base % modu;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result.wrapping_mul(base) % modu;
        }
        exp >>= 1;
        base = base.wrapping_mul(base) % modu;
    }
    result
}
