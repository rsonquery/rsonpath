import pandas as pd
import matplotlib.pyplot as plt

def plot_binned_frequencies(df: pd.DataFrame, save_path: str) -> None:
    if df.shape[1] != 2:
        raise ValueError("DataFrame should have exactly two columns: 'distance' and 'frequency'")
    
    # Build bins
    bin_edges = [0] + [2**i for i in range(1, 41)]
    df['binned_distance'] = pd.cut(df['distance'], bins=bin_edges, right=False, include_lowest=True)
    binned_df = df.groupby('binned_distance').agg({'frequency': 'sum'}).reset_index()
    binned_df['frequency'] = binned_df['frequency'].fillna(0)

    # Calculate total frequency
    total_frequency = binned_df['frequency'].sum()
    binned_df['percentage'] = (binned_df['frequency'] / total_frequency) * 100

    # Plotting the binned distances
    plt.figure(figsize=(12, 8))
    bars = plt.bar(binned_df['binned_distance'].astype(str), binned_df['percentage'], color='skyblue')
    plt.xticks(rotation=90)  # Rotate x-axis labels for better readability
    plt.xlabel('Distance (Binned)')
    plt.ylabel('Percentage of Total Frequency')
    plt.title(f'Distance vs Percentage of Total Frequency\n'
              f'Sum of all Frequencies: {total_frequency}, Max Distance: {df['distance'].max()}')
    plt.grid(True)

    # Add labels over each bar
    for bar in bars:
        yval = bar.get_height()
        plt.text(bar.get_x() + bar.get_width() / 2, yval, f'{int(yval * total_frequency / 100)}', 
                 ha='center', va='bottom', fontsize=10, color="blue", rotation=90)

    # Add vertical red line
    index_2_17 = binned_df[binned_df['binned_distance'].astype(str).str.contains('131072')].index[0]
    plt.axvline(x=index_2_17 - 0.5, color='red', linestyle='--', linewidth=2, label='2^17')

    plt.legend()

    # Save plot
    plt.tight_layout()
    plt.savefig(save_path)
    plt.close()

