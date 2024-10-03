import pandas as pd
import matplotlib.pyplot as plt
import os
import glob
import argparse
import numpy as np
from tqdm import tqdm

def read_csv_files(directory, file_pattern="*.csv"):
    """
    Reads all CSV files matching the file_pattern in the specified directory.
    Assumes each CSV file corresponds to a different algorithm.

    Parameters:
    - directory: Path to the directory containing CSV files.
    - file_pattern: Glob pattern to match CSV files.

    Returns:
    - A dictionary with keys as algorithm names (derived from filenames)
      and values as DataFrames containing 'prime' and 'timestamp'.
    """
    csv_files = glob.glob(os.path.join(directory, file_pattern))
    data_dict = {}

    for file_path in csv_files:
        # Derive algorithm name from filename
        algo_name = os.path.splitext(os.path.basename(file_path))[0].replace('_', ' ').title()
        try:
            df = pd.read_csv(file_path)
            if 'prime' not in df.columns or 'timestamp' not in df.columns:
                print(f"Warning: '{file_path}' does not contain required columns. Skipping.")
                continue
            # Convert timestamp to float
            df['timestamp'] = df['timestamp'].astype(float)
            # Sort by timestamp
            df = df.sort_values(by='timestamp')
            data_dict[algo_name] = df
        except Exception as e:
            print(f"Error reading '{file_path}': {e}")

    if not data_dict:
        print("No valid CSV files found. Please check the directory and file formats.")
    return data_dict

def process_data(df, bin_size=None):
    """
    Processes the DataFrame to calculate cumulative primes over time.
    Optionally aggregates data into time bins to handle large datasets efficiently.

    Parameters:
    - df: DataFrame with 'prime' and 'timestamp' columns.
    - bin_size: Size of each time bin in seconds. If None, no binning is applied.

    Returns:
    - A DataFrame with 'time' and 'cumulative_primes'.
    """
    if df.empty:
        return pd.DataFrame(columns=['time', 'cumulative_primes'])

    if bin_size:
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
        counts.rename(columns={'bin': 'time'}, inplace=True)

        return counts[['time', 'cumulative_primes']]
    else:
        # No binning; calculate cumulative primes directly
        df = df.sort_values('timestamp')
        df['cumulative_primes'] = range(1, len(df) + 1)
        return df[['timestamp', 'cumulative_primes']].rename(columns={'timestamp': 'time'})

def plot_primes(data_dict, output_filename, bin_size=None, desired_max_points=10000):
    """
    Plots the cumulative number of primes over time for each algorithm.
    Aggregates data into time bins if bin_size is specified.
    Saves the plot to the specified output filename.

    Parameters:
    - data_dict: Dictionary with algorithm names as keys and DataFrames as values.
    - output_filename: Name of the output image file (e.g., 'primes_over_time.png').
    - bin_size: Size of each time bin in seconds. If None, no binning is applied.
    - desired_max_points: Desired maximum number of points per algorithm (used if bin_size is None).
    """
    plt.figure(figsize=(14, 8))

    # Define colors and styles for better distinction
    colors = {
        'Trial Division': 'blue',
        'Sieve Of Eratosthenes': 'green',
        'Sieve Of Atkin': 'red',
        'Miller-Rabin': 'purple'
    }

    linestyles = {
        'Trial Division': '-',
        'Sieve Of Eratosthenes': '--',
        'Sieve Of Atkin': '-.',
        'Miller-Rabin': ':'
    }

    for algo, df in tqdm(data_dict.items(), desc="Processing Algorithms"):
        if df.empty:
            print(f"Warning: No data to plot for '{algo}'.")
            continue

        # Determine bin size if not provided
        if not bin_size:
            total_time = df['timestamp'].max()
            bin_size_calc = total_time / desired_max_points
            bin_size_calc = max(bin_size_calc, 0.1)  # Minimum bin size of 0.1 seconds
            processed_df = process_data(df, bin_size=bin_size_calc)
        else:
            processed_df = process_data(df, bin_size=bin_size)

        # Plot cumulative primes over time
        plt.step(processed_df['time'], processed_df['cumulative_primes'], where='post',
                 label=algo, color=colors.get(algo, None),
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
    # Set up argument parsing for flexibility
    parser = argparse.ArgumentParser(description="Graph the number of primes found over time.")
    parser.add_argument('--directory', type=str, default='.',
                        help='Directory containing CSV files (default: current directory)')
    parser.add_argument('--output', type=str, default='primes_over_time.png',
                        help='Output image filename (default: primes_over_time.png)')
    parser.add_argument('--bin_size', type=float, default=None,
                        help='Size of each time bin in seconds (default: auto)')
    parser.add_argument('--desired_points', type=int, default=10000,
                        help='Desired maximum number of data points per algorithm (used if bin_size is not set)')
    args = parser.parse_args()

    # Read CSV files
    data_dict = read_csv_files(args.directory)
    if not data_dict:
        print("No valid data to plot. Exiting.")
        return

    # Plot primes
    plot_primes(data_dict, args.output, bin_size=args.bin_size, desired_max_points=args.desired_points)

if __name__ == "__main__":
    main()
