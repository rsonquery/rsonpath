import sys
import os
import pandas as pd

import matplotlib.pyplot as plt

def plot_size(df: pd.DataFrame, save_path: str) -> None:
    df_sorted = df.sort_values(by=['input_size'])

    plt.figure(figsize=(10, 6))
    plt.plot(df_sorted['name'], df_sorted['naive'], marker='o', label='Naive')
    plt.plot(df_sorted['name'], df_sorted['distance'], marker='o', label='Distance')
    plt.plot(df_sorted['name'], df_sorted['perfect_naive'], marker='o', label='Perfect Naive')
    plt.plot(df_sorted['name'], df_sorted['phf'], marker='o', label='PHF')
    plt.plot(df_sorted['name'], df_sorted['phf_double'], marker='o', label='PHF Double')
    plt.plot(df_sorted['name'], df_sorted['phf_group'], marker='o', label='PHF Group')

    plt.title('Compare different lookup table heap sizes')
    plt.xlabel('Test Data Sets')
    plt.ylabel('Allocations in bytes')
    plt.xticks(rotation=90)
    plt.legend()
    plt.grid(True)
    plt.tight_layout()

    plt.savefig(save_path)

if __name__ == "__main__":
    # Load the CSV file
    file_path = sys.argv[1]
    directory, filename = os.path.split(file_path)
    file_base_name = os.path.splitext(filename)[0]

    # Plot
    df = pd.read_csv(sys.argv[1])
    plot_size(df, os.path.join(directory, f"{file_base_name}_heap_size.png"))
