# Prime Finder

Prime Finder is a command-line tool that finds prime numbers using various algorithms. It allows you to specify the algorithm to use, run algorithms for a specified amount of time, or run them continuously to find larger primes.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
    - [Command-Line Options](#command-line-options)
        - [`--algorithm`, `-a`](#--algorithm--a)
        - [`--max`, `-m`](#--max--m)
        - [`--time`, `-t`](#--time--t)
    - [Examples](#examples)
- [Algorithms](#algorithms)
    - [Sieve of Eratosthenes](#sieve-of-eratosthenes)
    - [Trial Division](#trial-division)
    - [Sieve of Atkin](#sieve-of-atkin)
    - [Miller-Rabin Primality Test](#miller-rabin-primality-test)
- [Output](#output)
- [Contributing](#contributing)
- [License](#license)

## Features

- **Multiple Algorithms**: Choose from various prime-finding algorithms:
    - Sieve of Eratosthenes
    - Trial Division
    - Sieve of Atkin
    - Miller-Rabin Primality Test
- **Time-Limited Execution**: Run algorithms for a specified amount of time.
- **Continuous Execution**: Run algorithms continuously to find larger primes.
- **Progress Indicators**: Visual progress bars to monitor the computation.
- **CSV Export**: Export statistics to CSV files for analysis.

## Installation

Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your system.

Clone the repository and build the project:

```bash
git clone https://github.com/yourusername/prime-finder.git
cd prime-finder
cargo build --release
```

## Usage

Run the `prime_finder` executable with the desired options.

## Command-Line Options

### `--algorithm`, `-a`
Specify the algorithm to use for finding primes.

Available algorithms:
- `eratosthenes`: Sieve of Eratosthenes
- `trial`: Trial Division
- `atkin`: Sieve of Atkin
- `miller-rabin`: Miller-Rabin Primality Test

**Usage:**
```bash
./target/release/prime_finder -a [algorithm]
```

**Example:**
```bash
./target/release/prime_finder -a trial
```

### `--max`, `-m`
Run the algorithm continuously to find larger primes.

**Usage:**
```bash
./target/release/prime_finder -a [algorithm] -m
```

**Example:**
```bash
./target/release/prime_finder -a eratosthenes -m
```

### `--time`, `-t`
Specify the time in seconds for which the algorithm(s) should run. If no algorithm is specified, all algorithms will run sequentially for the specified time.

**Usage:**
```bash
./target/release/prime_finder -t [seconds]
```

**Example:**
```bash
./target/release/prime_finder -t 30
```

## Examples

Run a specific algorithm once:
```bash
./target/release/prime_finder -a miller-rabin
```

Run an algorithm continuously:
```bash
./target/release/prime_finder -a atkin -m
```

Run an algorithm for a specified time:
```bash
./target/release/prime_finder -a trial -t 60
```

Run all algorithms for a specified time:
```bash
./target/release/prime_finder -t 45
```

## Algorithms

### Sieve of Eratosthenes
An ancient algorithm for finding all prime numbers up to a specified integer. It works efficiently by eliminating multiples of primes.

### Trial Division
A straightforward method that tests each candidate number for divisibility by all smaller integers up to its square root.

### Sieve of Atkin
A modern algorithm that is more efficient than the Sieve of Eratosthenes for large ranges. It uses mathematical properties to eliminate non-primes.

### Miller-Rabin Primality Test
A probabilistic test to determine if a number is likely prime. It is particularly useful for very large numbers.

## Output

The program outputs statistics for each algorithm, including:
- **Algorithm Name**: The name of the algorithm used.
- **Time Taken**: Duration of execution.
- **Largest Prime Found**: The largest prime number discovered.
- **Number of Primes Found**: Total count of prime numbers identified.

These statistics are exported to CSV files named after each algorithm, such as `sieve_of_eratosthenes_stats.csv`.



