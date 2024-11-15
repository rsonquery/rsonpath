import sys
import os
import pandas as pd

import matplotlib.pyplot as plt

def to_pretty_name(column_name: str, column_suffix: str) -> str:
    res = column_name.removesuffix(column_suffix)
    res = res.replace('_', ' ')
    res = res.upper()
    
    return res

def plot_columns(df: pd.DataFrame, save_path: str, column_suffix: str, title: str, ylabel: str) -> None:
    """
    Generic function to plot columns based on suffix (e.g., '_heap' or '_capacity').

    :param df: DataFrame containing the data
    :param save_path: Path to save the plot
    :param column_suffix: Suffix to filter columns (e.g., '_heap' or '_capacity')
    :param title: Title of the plot
    :param ylabel: Label for the y-axis
    """
    df = df.sort_values(by=['input_size_bytes'])
    df['label'] = df['name'] + "_" + df['num_keys'].astype(str)

    plt.figure(figsize=(10, 6))

    # Filter columns containing the specific suffix
    filtered_columns = [
        col for col in df.columns if col.endswith(column_suffix)]

    # Plot each filtered column
    for column in filtered_columns:
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
    plot_columns(
        df=df,
        save_path=save_path,
        column_suffix='_heap',
        title='Compare different lookup table heap sizes',
        ylabel='Allocations in bytes'
    )


def plot_capacity(df: pd.DataFrame, save_path: str) -> None:
    plot_columns(
        df=df,
        save_path=save_path,
        column_suffix='_capacity',
        title='Compare different lookup table capacity sizes',
        ylabel='Capacity in bytes'
    )


if __name__ == "__main__":
    # Load the CSV file
    file_path = sys.argv[1]
    dir, filename = os.path.split(file_path)
    file_base_name = os.path.splitext(filename)[0]

    # Plot
    df = pd.read_csv(sys.argv[1])
    plot_heap(df, os.path.join(dir, f"{file_base_name}_heap_size.png"))
    plot_capacity(df, os.path.join(dir, f"{file_base_name}_capacity.png"))
