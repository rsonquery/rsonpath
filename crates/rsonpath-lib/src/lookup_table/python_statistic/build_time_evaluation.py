import sys
import os
import pandas as pd
import matplotlib.pyplot as plt

def plot_build_time(df: pd.DataFrame, save_path: str) -> None:
    df = df.sort_values(by='num_keys')

    # Create a new column combining 'name' and 'num_keys'
    df['label'] = df['name'] + "_" + df['num_keys'].astype(str)

    # Set the plot size
    plt.figure(figsize=(10, 6))

    # Plot each column against the 'label' on x-axis
    plt.plot(df['label'], df['naive'], label='Naive', marker='o')
    plt.plot(df['label'], df['distance'], label='Distance', marker='o')
    plt.plot(df['label'], df['perfect_naive'], label='Perfect Naive', marker='o')
    plt.plot(df['label'], df['phf'], label='PHF', marker='o')
    plt.plot(df['label'], df['phf_double'], label='PHF Double', marker='o')
    plt.plot(df['label'], df['phf_group'], label='PHF Group', marker='o')

    # Set plot labels and title
    plt.xlabel('Dataset (Name_NumKeys)')
    plt.ylabel('Build Time (Seconds)')
    plt.title('Build Time Comparison by Dataset')

    # Rotate x-axis labels by 90 degrees
    plt.xticks(rotation=90)

    # Add grid and legend
    plt.grid(True)
    plt.legend()

    # Automatically adjust layout to prevent label cut-off
    plt.tight_layout()

    # Save the figure
    plt.savefig(save_path)
    
def plot_build_time_per_key(df: pd.DataFrame, save_path: str) -> None:
    df = df.sort_values(by='num_keys')

    # Create a new column combining 'name' and 'num_keys'
    df['label'] = df['name'] + "_" + df['num_keys'].astype(str)

    # Set the plot size
    plt.figure(figsize=(10, 6))

    # Plot each column against the 'label' on x-axis
    plt.plot(df['label'], df['naive'] / df['num_keys'], label='Naive', marker='o')
    plt.plot(df['label'], df['distance'] / df['num_keys'], label='Distance', marker='o')
    plt.plot(df['label'], df['perfect_naive'] / df['num_keys'], label='Perfect Naive', marker='o')
    plt.plot(df['label'], df['phf'] / df['num_keys'], label='PHF', marker='o')
    plt.plot(df['label'], df['phf_double'] / df['num_keys'], label='PHF Double', marker='o')
    plt.plot(df['label'], df['phf_group'] / df['num_keys'], label='PHF Group', marker='o')

    # Set plot labels and title
    plt.xlabel('Dataset (Name_NumKeys)')
    plt.ylabel('Build Time (Seconds) / NumKeys')
    plt.title('Build Time per Key Comparison by Dataset')

    # Rotate x-axis labels by 90 degrees
    plt.xticks(rotation=90)

    # Add grid and legend
    plt.grid(True)
    plt.legend()

    # Automatically adjust layout to prevent label cut-off
    plt.tight_layout()

    # Save the figure
    plt.savefig(save_path)

def plot_get_time(df: pd.DataFrame, save_path: str) -> None:
    df = df.sort_values(by='num_keys')
    
    df['label'] = df['name'] + "_" + df['num_keys'].astype(str)

    # Set the plot size
    plt.figure(figsize=(10, 6))

    # Plot each column against the 'name' on x-axis
    plt.plot(df['label'], df['naive_query'], label='Naive', marker='o')
    plt.plot(df['label'], df['distance_query'], label='Distance', marker='o')
    plt.plot(df['label'], df['perfect_naive_query'], label='Perfect Naive', marker='o')
    plt.plot(df['label'], df['phf_query'], label='PHF', marker='o')
    plt.plot(df['label'], df['phf_double_query'], label='PHF Double', marker='o')
    plt.plot(df['label'], df['phf_group_query'], label='PHF Group', marker='o')

    # Set plot labels and title
    plt.xlabel('Dataset (Name_NumKeys)')
    plt.ylabel('Get Time (Seconds)')
    plt.title('Get Time Comparison by Dataset')

    # Rotate x-axis labels by 90 degrees
    plt.xticks(rotation=90)

    # Add grid and legend
    plt.grid(True)
    plt.legend()

    # Automatically adjust layout to prevent label cut-off
    plt.tight_layout()

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
    plot_build_time_per_key(df, os.path.join(directory, f"{file_base_name}_build_time_per_key.png"))
    plot_get_time(df, os.path.join(directory, f"{file_base_name}_get_time.png"))
