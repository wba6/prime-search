import pandas as pd
import matplotlib.pyplot as plt
import os

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

def process_data(df):
    """
    Processes the DataFrame to calculate cumulative primes over time.
    Returns a DataFrame with timestamp and cumulative count.
    """
    if df is None or df.empty:
        return pd.DataFrame(columns=['timestamp', 'cumulative_primes'])
    # Create a DataFrame with unique timestamps and counts
    counts = df.groupby('timestamp').size().reset_index(name='count')
    # Sort by timestamp
    counts = counts.sort_values('timestamp')
    # Calculate cumulative sum
    counts['cumulative_primes'] = counts['count'].cumsum()
    return counts[['timestamp', 'cumulative_primes']]

def plot_primes(data_dict, output_filename):
    """
    Plots the cumulative number of primes over time for each algorithm.
    Saves the plot to the specified output filename.
    """
    plt.figure(figsize=(12, 8))

    for algo, data in data_dict.items():
        if data.empty:
            print(f"Warning: No data to plot for '{algo}'.")
            continue
        plt.plot(data['timestamp'], data['cumulative_primes'], label=algo)

    plt.title('Number of Primes Found Over Time')
    plt.xlabel('Time Elapsed (seconds)')
    plt.ylabel('Cumulative Number of Primes Found')
    plt.legend()
    plt.grid(True)
    plt.tight_layout()
    plt.savefig(output_filename)
    plt.close()
    print(f"Graph saved as '{output_filename}'.")

def main():
    # Define the CSV filenames
    algorithms = {
        'Trial Division': 'trial_division.csv',
        'Sieve of Eratosthenes': 'sieve_of_eratosthenes.csv',
        'Sieve of Atkin': 'sieve_of_atkin.csv',
        'Miller-Rabin': 'miller_rabin.csv'
    }

    # Read and process data for each algorithm
    data_dict = {}
    for algo, filename in algorithms.items():
        print(f"Processing '{algo}' from '{filename}'...")
        df = read_csv_file(filename)
        processed_data = process_data(df)
        data_dict[algo] = processed_data

    # Plot the data
    output_image = 'prime_comparison.png'
    plot_primes(data_dict, output_image)

if __name__ == "__main__":
    main()
