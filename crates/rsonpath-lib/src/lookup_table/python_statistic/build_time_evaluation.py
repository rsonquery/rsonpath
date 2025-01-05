import sys
import os
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt

PLOT_COLORS = [
    'red', 'black', 'blue', 'brown', 'green',
    'orange', 'purple', 'pink', 'cyan', 'magenta',
    'yellow', 'darkblue', 'darkgreen', 'darkred', 'darkcyan',
    'darkorange', 'darkviolet', 'lime', 'indigo', 'gold',
    'turquoise', 'chartreuse', 'teal', 'coral', 'salmon',
    'darkkhaki', 'crimson', 'peru', 'forestgreen', 'mediumpurple', 'skyblue'
]

def to_pretty_name(column_name: str, column_suffix: str) -> str:
    res = column_name.removesuffix(column_suffix)
    res = res.replace('_', ' ')
    return res


def plot_all(df: pd.DataFrame, save_path: str) -> None: 
    df = df.sort_values(by='num_keys')

    # Initialize the figure with subplots (2 columns for 8 plots)
    fig, axes = plt.subplots(4, 2, figsize=(18, 24), sharex=True)

    # Flatten axes for easier iteration
    axes = axes.flatten()

    # Define the configurations for the plots
    plot_configs = [
        {
            'column_suffix': '_build_time',
            'ylabel': 'Build time in seconds',
            'per_key': False
        },
        {
            'column_suffix': '_query_time',
            'ylabel': 'Sum of query time in seconds',
            'per_key': False
        },
        {
            'column_suffix': '_heap',
            'ylabel': 'Heap size in bytes',
            'per_key': False
        },
        {
            'column_suffix': '_capacity',
            'ylabel': 'Capacity in bytes',
            'per_key': False
        },
        {
            'column_suffix': '_build_time',
            'ylabel': 'Average build time in seconds per key',
            'per_key': True
        },
        {
            'column_suffix': '_query_time',
            'ylabel': 'Average query time in seconds per key',
            'per_key': True
        },
        {
            'column_suffix': '_heap',
            'ylabel': 'Heap size in bytes per key',
            'per_key': True
        },
        {
            'column_suffix': '_capacity',
            'ylabel': 'Capacity in bytes per key',
            'per_key': True
        },
    ]

    # Reshape axes into 2D array
    axes = axes.reshape(4, 2)

    col1_axes = axes[:, 0]  # First column of subplots
    col2_axes = axes[:, 1]  # Second column of subplots

    # To collect handles and labels for the legend
    handles, labels = [], []

    # Iterate over the plots
    for ax, config in zip(np.concatenate((col1_axes, col2_axes)), plot_configs):
        column_suffix = config['column_suffix']
        per_key = config['per_key']
        filtered_columns = [col for col in df.columns if col.endswith(column_suffix)]

        for (i, column) in enumerate(filtered_columns):
            name = to_pretty_name(column, column_suffix)

            if per_key:
                line, = ax.plot(df['num_keys'], df[column] / df['num_keys'], alpha=0.6, marker='o', label=name, color=PLOT_COLORS[i])
            else:
                line, = ax.plot(df['num_keys'], df[column], alpha=0.6, marker='o', label=name, color=PLOT_COLORS[i])

            # Collect handles and labels only once for each unique label
            if name not in labels:
                handles.append(line)
                labels.append(name)

        ax.set_ylabel(config['ylabel'], fontweight='bold', fontsize=12)
        ax.grid(True)

    # Add column titles
    col1_axes[0].set_title("Absolute Values", fontweight='bold', fontsize=14)
    col2_axes[0].set_title("Per Key Values", fontweight='bold', fontsize=14)

    # Configure shared x-axis to only mark real used values for all plots
    for ax in np.concatenate((col1_axes, col2_axes)):
        ax.set_xticks(df['num_keys'])
        ax.set_xticklabels(df['num_keys'], rotation=45)

    # Add x-axis label for the last row in both columns
    for ax in [col1_axes[-1], col2_axes[-1]]:
        ax.set_xlabel('Number of keys', fontweight='bold', fontsize=12)

    # Place legend only once outside of the plots
    fig.text(0.86, 0.85, 'Strategies', horizontalalignment='left', verticalalignment='top', style='italic', fontsize=20, bbox={'facecolor': (0.7, 0.8, 1),  # Light blue
                                                                                                                               'boxstyle': 'round'})
    fig.legend(handles, labels, loc='upper left', bbox_to_anchor=(
        0.86, 0.83), fontsize=10, fancybox=True, shadow=True)

    # Add text box under the legend (adjust y-position to make room)
    fig.text(0.86, 0.52, 'Data', horizontalalignment='left', verticalalignment='top', style='italic', fontsize=20, bbox={'facecolor': (0.7, 0.8, 1),  # Light blue
                                                                                                                         'boxstyle': 'round'})
    text_content = "\n".join(
        f"{name} : {num_keys}" for name, num_keys in zip(df['name'], df['num_keys']))
    fig.text(0.86, 0.5, text_content, horizontalalignment='left', verticalalignment='top', fontsize=10,
             bbox={'facecolor': 'white', 'boxstyle': 'round'})

    # Shrink the plot to make room for the legend and text box
    plt.subplots_adjust(right=0.85)

    plt.savefig(save_path)


if __name__ == "__main__":
    # Load the CSV file
    file_path = sys.argv[1]
    directory, filename = os.path.split(file_path)
    file_base_name = os.path.splitext(filename)[0]

    # Plot
    df = pd.read_csv(file_path)
    plot_all(df, os.path.join(
        directory, f"{file_base_name}_combined_plot.png"))
