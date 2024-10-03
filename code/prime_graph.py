import pandas as pd
import matplotlib.pyplot as plt
import os
import numpy as np
from tqdm import tqdm

def read_csv_file(filename):
    """
    Reads a CSV file and returns a DataFrame sorted by timestamp.
    """
    if not os.path.isfile(filename):
        print(f"Error: File '{filename}' not found.")
        return None
    try:
        df = pd.read_csv(filename)
        # Convert timestamp to float
        df['timestamp'] = df['timestamp'].astype(float)
        # Sort by timestamp
        df = df.sort_values(by='timestamp')
        return df
    except Exception as e:
        print(f"Error reading '{filename}': {e}")
        return None

def process_data(df, bin_size=1.0):
    """
    Processes the DataFrame to calculate cumulative primes over time.
    Aggregates data into time bins to handle large datasets efficiently.

    Parameters:
    - df: DataFrame with 'prime' and 'timestamp' columns.
    - bin_size: Size of each time bin in seconds (default: 1.0).

    Returns:
    - A DataFrame with 'bin_start' and 'cumulative_primes'.
    """
    if df is None or df.empty:
        return pd.DataFrame(columns=['bin_start', 'cumulative_primes'])

    # Define bin edges
    max_time = df['timestamp'].max()
    bins = np.arange(0, max_time + bin_size, bin_size)

    # Assign each prime to a bin
    df['bin'] = pd.cut(df['timestamp'], bins=bins, right=False, labels=bins[:-1])

    # Count primes in each bin
    counts = df.groupby('bin').size().reset_index(name='count')

    # Ensure all bins are represented
    all_bins = pd.DataFrame({'bin': bins[:-1]})
    counts = pd.merge(all_bins, counts, on='bin', how='left').fillna(0)

    # Sort by bin
    counts = counts.sort_values('bin')

    # Calculate cumulative sum
    counts['cumulative_primes'] = counts['count'].cumsum()

    # Rename columns for clarity
    counts.rename(columns={'bin': 'bin_start'}, inplace=True)

    return counts[['bin_start', 'cumulative_primes']]

def plot_primes(data_dict, output_filename, bin_size=1.0):
    """
    Plots the cumulative number of primes over time for each algorithm.
    Aggregates data into time bins to handle large datasets efficiently.
    Saves the plot to the specified output filename.

    Parameters:
    - data_dict: Dictionary with algorithm names as keys and processed DataFrames as values.
    - output_filename: Name of the output image file (e.g., 'prime_comparison.png').
    - bin_size: Size of each time bin in seconds.
    """
    plt.figure(figsize=(14, 8))

    # Define colors and styles for better distinction
    colors = {
        'Trial Division': 'blue',
        'Sieve of Eratosthenes': 'green',
        'Sieve of Atkin': 'red',
        'Miller-Rabin': 'purple'
    }

    linestyles = {
        'Trial Division': '-',
        'Sieve of Eratosthenes': '--',
        'Sieve of Atkin': '-.',
        'Miller-Rabin': ':'
    }

    for algo, data in tqdm(data_dict.items(), desc="Plotting Algorithms"):
        if data.empty:
            print(f"Warning: No data to plot for '{algo}'.")
            continue
        plt.plot(data['bin_start'], data['cumulative_primes'], label=algo,
                 color=colors.get(algo, None),
                 linestyle=linestyles.get(algo, '-'))

    plt.title('Number of Primes Found Over Time', fontsize=16)
    plt.xlabel('Time Elapsed (seconds)', fontsize=14)
    plt.ylabel('Cumulative Number of Primes Found', fontsize=14)
    plt.legend(title='Algorithms', fontsize=12)
    plt.grid(True, which='both', linestyle='--', linewidth=0.5)
    plt.tight_layout()
    plt.savefig(output_filename, dpi=300)
    plt.close()
    print(f"\nGraph saved as '{output_filename}'.\n")

def main():
    # Define the CSV filenames
    algorithms = {
        'Trial Division': 'trial_division.csv',
        'Sieve of Eratosthenes': 'sieve_of_eratosthenes.csv',
        'Sieve of Atkin': 'sieve_of_atkin.csv',
        'Miller-Rabin': 'miller_rabin.csv'
    }

    # Define bin size in seconds (adjust based on dataset size)
    bin_size = 1.0  # 1 second

    # Read and process data for each algorithm
    data_dict = {}
    for algo, filename in algorithms.items():
        print(f"Processing '{algo}' from '{filename}'...")
        df = read_csv_file(filename)
        processed_data = process_data(df, bin_size=bin_size)
        data_dict[algo] = processed_data

    # Plot the data
    output_image = 'prime_comparison_optimized.png'
    plot_primes(data_dict, output_image, bin_size=bin_size)

if __name__ == "__main__":
    main()
