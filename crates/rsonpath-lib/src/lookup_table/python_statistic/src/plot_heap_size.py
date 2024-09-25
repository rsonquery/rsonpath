import pandas as pd
import matplotlib.pyplot as plt


def plot_size(df: pd.DataFrame, save_path: str) -> None:
    df_sorted = df.sort_values(by=['input_size'])

    plt.figure(figsize=(10, 6))
    plt.plot(df_sorted['name'], df_sorted['naive_allocations'], marker='o', label='Naive')
    plt.plot(df_sorted['name'], df_sorted['distance_allocations'], marker='o', label='Distance')
    plt.plot(df_sorted['name'], df_sorted['perfect_naive_allocations'], marker='o', label='Perfect Naive')
    plt.plot(df_sorted['name'], df_sorted['phf_allocations'], marker='o', label='PHF')

    plt.title('Compare different lookup table heap sizes')
    plt.xlabel('Test Data Sets')
    plt.ylabel('Allocations in bytes')
    plt.xticks(rotation=90)
    plt.legend()
    plt.grid(True)
    plt.tight_layout()

    plt.savefig(save_path)
