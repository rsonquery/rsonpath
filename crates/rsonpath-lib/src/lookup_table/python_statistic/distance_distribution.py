import sys
import os
import pandas as pd
import matplotlib.pyplot as plt


def plot_binned_frequencies(df: pd.DataFrame, directory: str, file_base_name: str) -> None:
    if df.shape[1] != 2:
        raise ValueError(
            "DataFrame should have exactly two columns: 'distance' and 'frequency'")

    # Build bins
    bin_edges = [0] + [2**i for i in range(1, 41)]
    df['binned_distance'] = pd.cut(
        df['distance'], bins=bin_edges, right=False, include_lowest=True)
    binned_df = df.groupby('binned_distance').agg(
        {'frequency': 'sum'}).reset_index()
    binned_df['frequency'] = binned_df['frequency'].fillna(0)

    # Calculate total frequency
    max_distance = df["distance"].max()
    total_frequency = binned_df['frequency'].sum()
    binned_df['percentage'] = (binned_df['frequency'] / total_frequency) * 100

    # Plotting the binned distances
    plt.figure(figsize=(12, 8))
    bars = plt.bar(binned_df['binned_distance'].astype(
        str), binned_df['percentage'], color='skyblue')
    plt.xticks(rotation=90)  # Rotate x-axis labels for better readability
    plt.xlabel('Distance (Binned)')
    plt.ylabel('Percentage of Total Frequency')
    plt.title(f'Distance Distribution in {file_base_name}\n'
              f'Sum of all Frequencies: {total_frequency}, Max Distance: {max_distance}')
    plt.grid(True)

    # Add labels over each bar
    for bar in bars:
        yval = bar.get_height()
        plt.text(bar.get_x() + bar.get_width() / 2, yval, f'{int(yval * total_frequency / 100)}',
                 ha='center', va='bottom', fontsize=10, color="blue", rotation=90)

    # Add vertical red line
    index_2_17 = binned_df[binned_df['binned_distance'].astype(
        str).str.contains('131072')].index[0]
    plt.axvline(x=index_2_17 - 0.5, color='red',
                linestyle='--', linewidth=2, label='2^17')

    # Add vertical red line
    index_2_33 = binned_df[binned_df['binned_distance'].astype(
        str).str.contains('8589934592')].index[0]
    plt.axvline(x=index_2_33 - 0.5, color='orange',
                linestyle='--', linewidth=2, label='2^33')

    plt.legend()

    # Save plot
    plt.tight_layout()
    save_path = os.path.join(directory, f"{file_base_name}_plot.png")
    plt.savefig(save_path)
    plt.close()


def plot_binned_frequencies_short(df: pd.DataFrame, directory: str, file_base_name: str) -> None:
    # if df.shape[1] != 2:
    #     raise ValueError(
    #         "DataFrame should have exactly two columns: 'distance' and 'frequency'")

    bin_edges = [0] + [2**i for i in range(1, 41)]
    df['binned_distance'] = pd.cut(
        df['distance'], bins=bin_edges, right=False, include_lowest=True)
    binned_df = df.groupby('binned_distance').agg(
        {'frequency': 'sum'}).reset_index()
    binned_df['frequency'] = binned_df['frequency'].fillna(0)

    total_frequency = binned_df['frequency'].sum()
    binned_df['percentage'] = (binned_df['frequency'] / total_frequency) * 100

    # Trim to first 34 bins (i.e., 2^1 to 2^34)
    trimmed_df = binned_df.iloc[:34].copy()
    trimmed_df['bin_number'] = range(1, 35)

    # Plotting
    plt.figure(figsize=(12, 8))
    bars = plt.bar(trimmed_df['bin_number'],
                   trimmed_df['percentage'], color='skyblue')
    plt.xlabel('Bin Number')
    plt.ylabel('Percentage of Total Frequency')
    plt.grid(True)

    # Labels over bars
    for bar, freq in zip(bars, trimmed_df['frequency']):
        yval = bar.get_height()
        plt.text(bar.get_x() + bar.get_width() / 2, yval, f'{int(freq)}',
                 ha='center', va='bottom', fontsize=10, color="blue", rotation=90)

    # Add vertical lines at bin 17 and 33
    plt.axvline(x=17 - 0.5, color='red', linestyle='--',
                linewidth=2, label='2^17')
    plt.axvline(x=33 - 0.5, color='orange',
                linestyle='--', linewidth=2, label='2^33')

    plt.xticks(ticks=range(1, 35))
    plt.legend()

    plt.tight_layout()
    save_path = os.path.join(directory, f"{file_base_name}-short.png")
    plt.savefig(save_path)
    plt.close()


if __name__ == "__main__":
    # Load the CSV file
    file_path = sys.argv[1]
    directory, filename = os.path.split(file_path)
    file_base_name = os.path.splitext(filename)[0]

    # Plot
    df = pd.read_csv(file_path)
    base_name = file_base_name.removesuffix("_distances")
    plot_binned_frequencies(df, directory, base_name)
    plot_binned_frequencies_short(df, directory, base_name)

    # TODO I want to create another plot as the one in plot_binned_frequencies
    # but I want no title and I want the x-axis labels to be 1, 2, 3, ... instead of the bin ranges
    # also make the x-axis from 1 to 34, so stop a little earlier
    # save this plot <name_of_the_other_plot>-short.png
