import sys
import os
import pandas as pd

import matplotlib.pyplot as plt

def to_pretty_name(column_name: str, column_suffix: str) -> str:
    res = column_name.removesuffix(column_suffix)
    res = res.replace('_', ' ')
    # res = res.upper()
    
    return res

def plot_size(df: pd.DataFrame, save_path: str, column_suffix: str, title: str, ylabel: str, per_key: bool) -> None:
    df = df.sort_values(by=['num_keys'])
    df['label'] = df['name'] + "_" + df['num_keys'].astype(str)

    plt.figure(figsize=(10, 6))

    # Filter columns containing the specific suffix
    filtered_columns = [
        col for col in df.columns if col.endswith(column_suffix)]

    # Plot each filtered column
    for column in filtered_columns:
        if per_key:
            plt.plot(df['label'], df[column] / df['num_keys'], marker='o', label=to_pretty_name(column, column_suffix))
        else:    
            plt.plot(df['label'], df[column], marker='o', label=to_pretty_name(column, column_suffix))

    plt.title(title)
    plt.xlabel('Test Data Sets')
    plt.ylabel(ylabel)
    plt.xticks(rotation=90)
    plt.legend()
    plt.grid(True)
    plt.tight_layout()

    plt.savefig(save_path)


def plot_heap(df: pd.DataFrame, save_path: str) -> None:
    plot_size(
        df=df,
        save_path=save_path,
        column_suffix='_heap',
        title='Compare lut heap',
        ylabel='Allocations in bytes',
        per_key=False,
    )

def plot_heap_per_key(df: pd.DataFrame, save_path: str) -> None:
    plot_size(
        df=df,
        save_path=save_path,
        column_suffix='_heap',
        title='Compare lut heap per key',
        ylabel='Allocations in bytes per key',
        per_key=True,
    )

def plot_capacity(df: pd.DataFrame, save_path: str) -> None:
    plot_size(
        df=df,
        save_path=save_path,
        column_suffix='_capacity',
        title='Compare lut capacity sizes',
        ylabel='Capacity in bytes',
        per_key=False,
    )

def plot_capacity_per_key(df: pd.DataFrame, save_path: str) -> None:
    plot_size(
        df=df,
        save_path=save_path,
        column_suffix='_capacity',
        title='Compare lut capacity sizes per key',
        ylabel='Capacity in bytes per key',
        per_key=True,
    )


if __name__ == "__main__":
    # Load the CSV file
    file_path = sys.argv[1]
    dir, filename = os.path.split(file_path)
    file_base_name = os.path.splitext(filename)[0]

    # Plot
    df = pd.read_csv(sys.argv[1])
    plot_heap(df, os.path.join(dir, f"{file_base_name}_heap_size.png"))
    plot_heap_per_key(df, os.path.join(dir, f"{file_base_name}_heap_size_per_key.png"))
    plot_capacity(df, os.path.join(dir, f"{file_base_name}_capacity.png"))
    plot_capacity_per_key(df, os.path.join(dir, f"{file_base_name}_capacity_per_key.png"))
