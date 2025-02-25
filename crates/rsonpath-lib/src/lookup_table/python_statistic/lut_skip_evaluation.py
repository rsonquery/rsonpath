import sys
import pandas as pd
import matplotlib.pyplot as plt

def plot(skip_timing_csv: str, skip_count_csv: str):
    df_times = pd.read_csv(skip_timing_csv)
    df_count = pd.read_csv(skip_count_csv)

    # Merge data on both "JSON" and "QUERY_NAME"
    df_merged = pd.merge(df_times, df_count, on=["FILENAME", "QUERY_NAME"])

    # Sort by LUT_PERCENT_SKIP
    df_sorted = df_merged.sort_values(by="LUT_PERCENT_SKIP")

    # Convert values for better readability
    build_time = df_sorted["T_LUT_BUILD"].iloc[0] / 1_000_000  # ns to ms
    capacity = df_sorted["LUT_CAPACITY"].iloc[0] / (1024 * 1024)  # bytes to MB

    # Create figure with two subplots
    fig, axes = plt.subplots(2, 1, figsize=(12, 10), sharex=True)

    # First plot: Execution Times
    df_sorted.plot(
        x="QUERY_NAME",
        y=["T_ORIGINAL", "T_LUT", "T_OPTIMUM", "T_ORIGINAL_SKIP", "T_LUT_SKIP"],
        kind="bar",
        ax=axes[0],
        color=["salmon", "blue", "green", "lightsalmon", "lightblue"]
    )
    axes[0].set_ylabel("Time (ns)")
    axes[0].set_title(f"Query Execution Time Comparison\nLUT Build Time: {build_time} ms, LUT Capacity: {capacity:.2f} MB")

    # Second plot: Skip Count
    df_sorted.plot(
        x="QUERY_NAME",
        y=["LUT_PERCENT_SKIP", "ITE_PERCENT_SKIP", "TOTAL_PERCENT_SKIP"],
        kind="bar",
        ax=axes[1],
        color=["purple", "orange", "cyan"]
    )
    axes[1].set_ylabel("Skip Percentage")
    axes[1].set_title("Skip Percentages per Query")

    # Adjust layout
    plt.xticks(rotation=45)
    plt.tight_layout()

    # Save the figure
    output_file = skip_timing_csv.replace(".csv", "_combined_sorted_plot.png")
    plt.savefig(output_file)
    plt.close()

if __name__ == "__main__":
    skip_timing_csv = sys.argv[1]  # First CSV (timing data)
    skip_count_csv = sys.argv[2]  # Second CSV (skip percentages)

    plot(skip_timing_csv, skip_count_csv)
