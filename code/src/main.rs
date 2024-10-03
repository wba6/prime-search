use clap::Parser;
use crossbeam::channel::{unbounded, tick, Sender};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::fs::File;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::{Duration, Instant};
use rand::Rng;
use csv::Writer;
use log::{error, info, warn};
use env_logger;

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
    // Initialize logging
    env_logger::init();

    // Parse command line arguments
    let args = Args::parse();
    let duration = Duration::from_secs(args.duration);
    println!(
        "Running prime algorithms for {} seconds...",
        args.duration
    );

    info!("Program started with duration: {} seconds", args.duration);

    // Shared stop flag
    let stop_flag = Arc::new(AtomicBool::new(false));

    // Channels for each algorithm
    let (td_sender, td_receiver) = unbounded();
    let (soe_sender, soe_receiver) = unbounded();
    let (soa_sender, soa_receiver) = unbounded();
    let (mr_sender, mr_receiver) = unbounded();

    // Create CSV writers
    let td_file = File::create(Algorithm::TrialDivision.csv_filename())?;
    let mut td_writer = Writer::from_writer(td_file);
    td_writer.write_record(&["prime", "timestamp"])?;

    let soe_file = File::create(Algorithm::SieveOfEratosthenes.csv_filename())?;
    let mut soe_writer = Writer::from_writer(soe_file);
    soe_writer.write_record(&["prime", "timestamp"])?;

    let soa_file = File::create(Algorithm::SieveOfAtkin.csv_filename())?;
    let mut soa_writer = Writer::from_writer(soa_file);
    soa_writer.write_record(&["prime", "timestamp"])?;

    let mr_file = File::create(Algorithm::MillerRabin.csv_filename())?;
    let mut mr_writer = Writer::from_writer(mr_file);
    mr_writer.write_record(&["prime", "timestamp"])?;

    // Set up progress bars
    let m = MultiProgress::new();
    let pb_style = ProgressStyle::with_template(
        "{spinner} {msg} | Elapsed: {elapsed_precise}", // Updated template
    )
        .unwrap() // Only one unwrap is needed
        .progress_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏");

    // Initialize progress bars as spinners
    let td_pb = m.add(ProgressBar::new_spinner());
    td_pb.set_style(pb_style.clone());
    td_pb.set_message(format!("{} | Largest Prime: 0", Algorithm::TrialDivision.as_str())); // Initial message

    let soe_pb = m.add(ProgressBar::new_spinner());
    soe_pb.set_style(pb_style.clone());
    soe_pb.set_message(format!(
        "{} | Largest Prime: 0",
        Algorithm::SieveOfEratosthenes.as_str()
    )); // Initial message

    let soa_pb = m.add(ProgressBar::new_spinner());
    soa_pb.set_style(pb_style.clone());
    soa_pb.set_message(format!(
        "{} | Largest Prime: 0",
        Algorithm::SieveOfAtkin.as_str()
    )); // Initial message

    let mr_pb = m.add(ProgressBar::new_spinner());
    mr_pb.set_style(pb_style.clone());
    mr_pb.set_message(format!(
        "{} | Largest Prime: 0",
        Algorithm::MillerRabin.as_str()
    )); // Initial message

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
        info!("Trial Division thread terminated.");
    });

    let soe_handle = thread::spawn(move || {
        sieve_of_eratosthenes(soe_sender, soe_stop);
        info!("Sieve of Eratosthenes thread terminated.");
    });

    let soa_handle = thread::spawn(move || {
        sieve_of_atkin(soa_sender, soa_stop);
        info!("Sieve of Atkin thread terminated.");
    });

    let mr_handle = thread::spawn(move || {
        miller_rabin(mr_sender, mr_stop);
        info!("Miller-Rabin thread terminated.");
    });

    // Track largest primes
    let mut largest_primes = vec![
        (Algorithm::TrialDivision, 0u64),
        (Algorithm::SieveOfEratosthenes, 0u64),
        (Algorithm::SieveOfAtkin, 0u64),
        (Algorithm::MillerRabin, 0u64),
    ];

    // Set up a ticker for periodic updates (every 100ms)
    let ticker = tick(Duration::from_millis(100));

    // Main loop to receive primes and write to CSV
    loop {
        let elapsed = start_time.elapsed();
        if elapsed >= duration {
            info!("Duration reached. Stopping algorithms.");
            break;
        }

        crossbeam::select! {
            recv(td_receiver) -> msg => {
                if let Ok(prime) = msg {
                    let timestamp = format!("{:.2}", elapsed.as_secs_f64());
                    if let Err(e) = td_writer.serialize((&prime, &timestamp)) {
                        error!("Failed to serialize Trial Division prime: {}", e);
                    }
                    if let Err(e) = td_writer.flush() {
                        error!("Failed to flush Trial Division writer: {}", e);
                    }
                    if prime > largest_primes[0].1 {
                        largest_primes[0].1 = prime;
                        td_pb.set_message(format!("{} | Largest Prime: {}", Algorithm::TrialDivision.as_str(), prime));
                    }
                }
            },
            recv(soe_receiver) -> msg => {
                if let Ok(prime) = msg {
                    let timestamp = format!("{:.2}", elapsed.as_secs_f64());
                    if let Err(e) = soe_writer.serialize((&prime, &timestamp)) {
                        error!("Failed to serialize Sieve of Eratosthenes prime: {}", e);
                    }
                    if let Err(e) = soe_writer.flush() {
                        error!("Failed to flush Sieve of Eratosthenes writer: {}", e);
                    }
                    if prime > largest_primes[1].1 {
                        largest_primes[1].1 = prime;
                        soe_pb.set_message(format!("{} | Largest Prime: {}", Algorithm::SieveOfEratosthenes.as_str(), prime));
                    }
                }
            },
            recv(soa_receiver) -> msg => {
                if let Ok(prime) = msg {
                    let timestamp = format!("{:.2}", elapsed.as_secs_f64());
                    if let Err(e) = soa_writer.serialize((&prime, &timestamp)) {
                        error!("Failed to serialize Sieve of Atkin prime: {}", e);
                    }
                    if let Err(e) = soa_writer.flush() {
                        error!("Failed to flush Sieve of Atkin writer: {}", e);
                    }
                    if prime > largest_primes[2].1 {
                        largest_primes[2].1 = prime;
                        soa_pb.set_message(format!("{} | Largest Prime: {}", Algorithm::SieveOfAtkin.as_str(), prime));
                    }
                }
            },
            recv(mr_receiver) -> msg => {
                if let Ok(prime) = msg {
                    let timestamp = format!("{:.2}", elapsed.as_secs_f64());
                    if let Err(e) = mr_writer.serialize((&prime, &timestamp)) {
                        error!("Failed to serialize Miller-Rabin prime: {}", e);
                    }
                    if let Err(e) = mr_writer.flush() {
                        error!("Failed to flush Miller-Rabin writer: {}", e);
                    }
                    if prime > largest_primes[3].1 {
                        largest_primes[3].1 = prime;
                        mr_pb.set_message(format!("{} | Largest Prime: {}", Algorithm::MillerRabin.as_str(), prime));
                    }
                }
            },
            recv(ticker) -> _ => {
                // Periodic updates or maintenance can be performed here if needed
            },
        }
    }

    // Capture the exact elapsed time at the moment of termination
    let final_elapsed = start_time.elapsed();

    // Signal threads to stop
    stop_flag.store(true, Ordering::SeqCst);
    info!("Stop flag set. Waiting for threads to terminate.");

    // Wait for threads to finish
    let handles = vec![td_handle, soe_handle, soa_handle, mr_handle];
    for handle in handles {
        if let Err(e) = handle.join() {
            error!("A thread encountered an error: {:?}", e);
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
    info!("Program terminated successfully.");

    Ok(())
}

/// Trial Division Algorithm
fn trial_division(sender: Sender<u64>, stop_flag: Arc<AtomicBool>) {
    let mut num: u64 = 2;
    while !stop_flag.load(Ordering::SeqCst) {
        if is_prime_trial_division(num) {
            if sender.send(num).is_err() {
                warn!("Trial Division receiver has been dropped.");
                break;
            }
        }
        num += 1;
    }
    info!("Trial Division algorithm stopped.");
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

    // Initial sieve processing
    while current < size {
        if stop_flag.load(Ordering::SeqCst) {
            info!("Sieve of Eratosthenes detected stop flag.");
            return;
        }
        if sieve[current] {
            if sender.send(current as u64).is_err() {
                warn!("Sieve of Eratosthenes receiver has been dropped.");
                return;
            }
            let mut multiple = current * 2;
            while multiple < size {
                if stop_flag.load(Ordering::SeqCst) {
                    info!("Sieve of Eratosthenes detected stop flag during sieving.");
                    return;
                }
                sieve[multiple] = false;
                multiple += current;
            }
        }
        current += 1;
    }

    // Dynamically increase sieve size
    loop {
        if stop_flag.load(Ordering::SeqCst) {
            info!("Sieve of Eratosthenes detected stop flag.");
            break;
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
                        info!("Sieve of Eratosthenes detected stop flag during dynamic sieving.");
                        return;
                    }
                    sieve[multiple] = false;
                    multiple += p;
                }
            }
        }

        // Sieve the new range
        while current < size {
            if stop_flag.load(Ordering::SeqCst) {
                info!("Sieve of Eratosthenes detected stop flag during sieving.");
                return;
            }
            if sieve[current] {
                if sender.send(current as u64).is_err() {
                    warn!("Sieve of Eratosthenes receiver has been dropped.");
                    return;
                }
                let mut multiple = current * 2;
                while multiple < size {
                    if stop_flag.load(Ordering::SeqCst) {
                        info!("Sieve of Eratosthenes detected stop flag during multiple elimination.");
                        return;
                    }
                    sieve[multiple] = false;
                    multiple += current;
                }
            }
            current += 1;
        }
    }
    info!("Sieve of Eratosthenes algorithm stopped.");
}

/// Complete Sieve of Atkin Implementation
fn sieve_of_atkin(sender: Sender<u64>, stop_flag: Arc<AtomicBool>) {
    let mut limit = 1000;
    let mut sieve = vec![false; (limit + 1) as usize];
    sieve[2] = true;
    sieve[3] = true;

    // Initial sieve processing
    for x in 1..=((limit as f64).sqrt() as u64) {
        for y in 1..=((limit as f64).sqrt() as u64) {
            if stop_flag.load(Ordering::SeqCst) {
                info!("Sieve of Atkin detected stop flag during initial processing.");
                return;
            }

            let n = 4 * x * x + y * y;
            if n <= limit && (n % 12 == 1 || n % 12 == 5) {
                sieve[n as usize] = !sieve[n as usize];
            }

            let n = 3 * x * x + y * y;
            if n <= limit && n % 12 == 7 {
                sieve[n as usize] = !sieve[n as usize];
            }

            if x > y {
                let n = 3 * x * x - y * y;
                if n <= limit && n % 12 == 11 {
                    sieve[n as usize] = !sieve[n as usize];
                }
            }
        }
    }

    // Eliminate composites by marking multiples of squares
    for r in 5..=((limit as f64).sqrt() as u64) {
        if sieve[r as usize] {
            let mut multiple = r * r;
            while multiple <= limit {
                sieve[multiple as usize] = false;
                multiple += r * r;
            }
        }
    }

    // Send primes up to the initial limit
    for num in 2..=limit {
        if sieve[num as usize] {
            if sender.send(num).is_err() {
                warn!("Sieve of Atkin receiver has been dropped.");
                return;
            }
        }
    }

    let mut current = limit + 1;

    // Dynamically increase sieve size
    loop {
        if stop_flag.load(Ordering::SeqCst) {
            info!("Sieve of Atkin detected stop flag.");
            break;
        }

        // Increase sieve size
        limit *= 2;
        sieve.resize((limit + 1) as usize, false);

        // Reapply sieve rules for the new range
        for x in 1..=((limit as f64).sqrt() as u64) {
            for y in 1..=((limit as f64).sqrt() as u64) {
                if stop_flag.load(Ordering::SeqCst) {
                    info!("Sieve of Atkin detected stop flag during dynamic processing.");
                    return;
                }

                let n = 4 * x * x + y * y;
                if n > limit {
                    continue;
                }
                if n % 12 == 1 || n % 12 == 5 {
                    sieve[n as usize] = !sieve[n as usize];
                }

                let n = 3 * x * x + y * y;
                if n > limit {
                    continue;
                }
                if n % 12 == 7 {
                    sieve[n as usize] = !sieve[n as usize];
                }

                if x > y {
                    let n = 3 * x * x - y * y;
                    if n <= limit && n % 12 == 11 {
                        sieve[n as usize] = !sieve[n as usize];
                    }
                }
            }
        }

        // Eliminate composites by marking multiples of squares
        for r in 5..=((limit as f64).sqrt() as u64) {
            if sieve[r as usize] {
                let mut multiple = r * r;
                while multiple <= limit {
                    sieve[multiple as usize] = false;
                    multiple += r * r;
                }
            }
        }

        // Send new primes in the extended range
        for num in current..=limit {
            if sieve[num as usize] {
                if sender.send(num as u64).is_err() {
                    warn!("Sieve of Atkin receiver has been dropped.");
                    return;
                }
            }
        }

        current = limit + 1;
    }
    info!("Sieve of Atkin algorithm stopped.");
}

/// Miller-Rabin Primality Test Algorithm with Improved Stop Flag Checking
fn miller_rabin(sender: Sender<u64>, stop_flag: Arc<AtomicBool>) {
    let mut num: u64 = 2;
    while !stop_flag.load(Ordering::SeqCst) {
        if is_prime_miller_rabin(num) {
            if sender.send(num).is_err() {
                warn!("Miller-Rabin receiver has been dropped.");
                break;
            }
        }
        num += 1;

        // Periodically check the stop flag
        if num % 1000 == 0 {
            if stop_flag.load(Ordering::SeqCst) {
                info!("Miller-Rabin detected stop flag.");
                break;
            }
        }
    }
    info!("Miller-Rabin algorithm stopped.");
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
    let mut result: u64 = 1;
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
