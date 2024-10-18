import sys
import os
import pandas as pd
import matplotlib.pyplot as plt

def plot_build_time(df: pd.DataFrame, save_path: str) -> None:
    # Sort the dataframe by input_size
    df = df.sort_values(by='input_size')

    # Set the plot size
    plt.figure(figsize=(10, 6))

    # Plot each column against the 'name' on x-axis
    plt.plot(df['name'], df['naive'], label='Naive', marker='o')
    plt.plot(df['name'], df['distance'], label='Distance', marker='o')
    plt.plot(df['name'], df['perfect_naive'], label='Perfect Naive', marker='o')
    plt.plot(df['name'], df['phf'], label='PHF', marker='o')
    plt.plot(df['name'], df['phf_double'], label='PHF Double', marker='o')
    plt.plot(df['name'], df['phf_group'], label='PHF Group', marker='o')

    # Set plot labels and title
    plt.xlabel('Dataset Name')
    plt.ylabel('Build Time (Seconds)')
    plt.title('Build Time Comparison by Dataset')

    # Rotate x-axis labels by 90 degrees
    plt.xticks(rotation=90)

    # Add grid and legend
    plt.grid(True)
    plt.legend()

    # Automatically adjust layout to prevent label cut-off
    plt.tight_layout()

    # Save the plot
    plt.savefig(save_path)

if __name__ == "__main__":
    # Load the CSV file
    file_path = sys.argv[1]
    directory, filename = os.path.split(file_path)
    file_base_name = os.path.splitext(filename)[0]

    # Read the CSV into a DataFrame
    df = pd.read_csv(file_path)

    # Plot and save the build time graph
    plot_build_time(df, os.path.join(directory, f"{file_base_name}_build_time.png"))
