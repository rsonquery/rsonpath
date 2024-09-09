import pandas as pd
import matplotlib.pyplot as plt


def plot_size(df: pd.DataFrame, save_path: str) -> None:
    """
    Plots the CBOR and JSON sizes from the dataframe and saves the plot as an image file.
    """
    df_sorted = df.sort_values(by=['input_size'])

    # Plotting cbor_size and json_size
    plt.figure(figsize=(10, 6))
    plt.plot(df_sorted['name'], df_sorted['naive_size'],
             marker='o', label='NAIVE Size')
    plt.plot(df_sorted['name'], df_sorted['distance_size'],
             marker='o', label='DISTANCE Size')

    plt.title('Naive Size vs. Distance Size')
    plt.xlabel('Test Data Sets')
    plt.ylabel('Size in Bytes')
    plt.xticks(rotation=45)
    plt.legend()
    plt.grid(True)
    plt.tight_layout()

    print(f"Saving statistic to {save_path}")
    plt.savefig(save_path)


def plot_speed(df: pd.DataFrame, save_path: str) -> None:
    """
    Plots the speed (input size / time) in GB/s for build, CBOR, and JSON operations from the dataframe and saves the 
    plot as an image file.
    """
    df = df.copy()
    # Convert input size from bytes to GB
    df['input_size_gb'] = df['input_size'] / (1024 ** 3)

    # Calculate speeds in GB/s
    df['naive_speed'] = df['input_size_gb'] / df['build_naive']
    df['distance_speed'] = df['input_size_gb'] / df['build_distance']

    df_sorted = df.sort_values(by=['input_size'])

    # Plotting speeds
    plt.figure(figsize=(10, 6))
    plt.plot(df_sorted['name'], df_sorted['naive_speed'], marker='o', label='Naive Speed')
    plt.plot(df_sorted['name'], df_sorted['distance_speed'], marker='o', label='Distance Speed')

    plt.title('Naive vs. Distance Speed in GB/s')
    plt.xlabel('Test Data Sets')
    plt.ylabel('Speed in GB/s')
    plt.xticks(rotation=45)
    plt.legend()
    plt.grid(True)
    plt.tight_layout()

    print(f"Saving statistic to {save_path}")
    plt.savefig(save_path)
