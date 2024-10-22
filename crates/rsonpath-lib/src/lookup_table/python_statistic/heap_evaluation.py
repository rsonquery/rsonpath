import sys
import os
import pandas as pd

import matplotlib.pyplot as plt

def plot_heap(df: pd.DataFrame, save_path: str) -> None:
    df = df.sort_values(by=['input_size'])
    df['label'] = df['name'] + "_" + df['num_keys'].astype(str)    

    plt.figure(figsize=(10, 6))
    plt.plot(df['label'], df['naive'], marker='o', label='Naive')
    # plt.plot(df['label'], df['perfect_naive'], marker='o', label='Perfect Naive')
    plt.plot(df['label'], df['phf'], marker='o', label='PHF')
    plt.plot(df['label'], df['phf_double'], marker='o', label='PHF Double')
    plt.plot(df['label'], df['phf_group'], marker='o', label='PHF Group')

    plt.title('Compare different lookup table heap sizes')
    plt.xlabel('Test Data Sets')
    plt.ylabel('Allocations in bytes')
    plt.xticks(rotation=90)
    plt.legend()
    plt.grid(True)
    plt.tight_layout()

    plt.savefig(save_path)
    
def plot_capacity(df: pd.DataFrame, save_path: str) -> None:
    df = df.sort_values(by=['input_size'])
    df['label'] = df['name'] + "_" + df['num_keys'].astype(str)    

    plt.figure(figsize=(10, 6))
    plt.plot(df['label'], df['naive_capacity'], marker='o', label='Naive')
    # plt.plot(df['label'], df['perfect_naive_capacity'], marker='o', label='Perfect Naive')
    plt.plot(df['label'], df['phf_capacity'], marker='o', label='PHF')
    plt.plot(df['label'], df['phf_double_capacity'], marker='o', label='PHF Double')
    plt.plot(df['label'], df['phf_group_capacity'], marker='o', label='PHF Group')

    plt.title('Compare different lookup table capacity sizes')
    plt.xlabel('Test Data Sets')
    plt.ylabel('Capacity in bytes')
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
    plot_heap(df, os.path.join(directory, f"{file_base_name}_heap_size.png"))
    plot_capacity(df, os.path.join(directory, f"{file_base_name}_capacity.png"))
