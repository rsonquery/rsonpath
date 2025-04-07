import pandas as pd
import matplotlib.pyplot as plt
import matplotlib.image as mpimg
import sys
import os

def plot_all(build_csv_path: str, query_csv_path: str, counter_csv_path: str, top_image_path):
    # Load CSV files
    query_df = pd.read_csv(query_csv_path)
    build_df = pd.read_csv(build_csv_path)
    counter_df = pd.read_csv(counter_csv_path)

    # Sort counter_df by TOTAL_PERCENT_SKIP & Filter to only include QUERY_NAME values that are in query_df["QUERY_ID"]
    counter_df = counter_df.sort_values("TOTAL_PERCENT_SKIP")
    counter_df = counter_df[counter_df["QUERY_NAME"].isin(query_df["QUERY_ID"])]

    # Add TOTAL_PERCENT_SKIP column & then sort query_df
    query_df = query_df.merge(
        counter_df[['QUERY_NAME', 'TOTAL_PERCENT_SKIP']], 
        left_on='QUERY_ID', 
        right_on='QUERY_NAME', 
        how='left'
    )
    query_df = query_df.sort_values("TOTAL_PERCENT_SKIP")

    # Sort & convert size to MB for build_df
    build_df = build_df.sort_values("CUTOFF")
    build_df["SIZE_MB"] = build_df["SIZE_IN_BYTES"] / (1024 * 1024)

    # Get output directory and filename for saving the plot
    output_dir = os.path.dirname(os.path.abspath(build_csv_path))
    json_filename = build_df["JSON"].iloc[0]
    output_filename = os.path.join(output_dir, f"{json_filename}.png")

    # Create a large figure with 2 rows and 2 columns for the subplots
    fig = plt.figure(figsize=(16, 14))

    # Load and display the top image
    if os.path.exists(top_image_path):
        img = mpimg.imread(top_image_path)
        ax_img = fig.add_axes([0, 0.6, 1, 0.4])
        ax_img.imshow(img)
        ax_img.axis('off')

    # Define color map for the cutoffs (to be consistent across plots)
    unique_cutoffs = query_df["CUTOFF"].unique()
    # Use 'tab10' colormap, adjust based on unique cutoffs
    colors = plt.cm.get_cmap('tab10', len(unique_cutoffs))
    cutoff_color_map = {cutoff: colors(i) for i, cutoff in enumerate(unique_cutoffs)}

    # Create subplots for the 4 plots (2 rows, 2 columns)
    # --- Plot 1 ---
    ax1 = fig.add_subplot(2, 2, 1) 
    for cutoff, group in query_df.groupby("CUTOFF"):
        ax1.plot(
            group["QUERY_ID"].astype(str), 
            group["QUERY_TIME_SECONDS"],
            marker='o', 
            label=f"Cutoff {cutoff}",
            color=cutoff_color_map[cutoff]
        )
    ax1.set_xlabel("Query ID")
    ax1.set_ylabel("Query Time (seconds)")
    ax1.set_title("Query Time by Query ID (Grouped by Cutoff)")
    ax1.legend(title="Cutoff")
    ax1.grid(True)
    ax1.set_xticklabels(group["QUERY_ID"].astype(str), rotation=90, fontsize=9)  

    # --- Plot 2: Build Time vs Cutoff (Bar Plot) ---
    ax2 = fig.add_subplot(2, 2, 2)  # Second subplot (1st row, 2nd column)
    ax2.bar(build_df["CUTOFF"].astype(str), build_df["BUILD_TIME_SECONDS"], color=[
            cutoff_color_map[cutoff] for cutoff in build_df["CUTOFF"]])
    ax2.set_xlabel("Cutoff")
    ax2.set_ylabel("Build Time (seconds)")
    ax2.set_title("Build Time by Cutoff")

    # --- Plot 3: Total Percent Skip by Query ID (Bar Plot) ---
    ax3 = fig.add_subplot(2, 2, 3)  # Third subplot (2nd row, 1st column)
    counter_df.plot(
        x="QUERY_NAME",
        y="TOTAL_PERCENT_SKIP",
        kind="bar",
        ax=ax3,
        color="#1f77b4"
    )
    ax3.set_xlabel("Query ID")
    ax3.set_ylabel("Total Percent Skip (%)")
    ax3.set_title("Total Percent Skip by Query ID")
    ax3.set_xticklabels(counter_df["QUERY_NAME"], rotation=90, fontsize=9)  

    # --- Plot 4: Size vs Cutoff (Bar Plot, in MB) ---
    ax4 = fig.add_subplot(2, 2, 4)  # Fourth subplot (2nd row, 2nd column)
    ax4.bar(build_df["CUTOFF"].astype(str), build_df["SIZE_MB"], color=[
            cutoff_color_map[cutoff] for cutoff in build_df["CUTOFF"]])
    ax4.set_xlabel("Cutoff")
    ax4.set_ylabel("LUT Size (MB)")
    ax4.set_title("LUT Size by Cutoff")

    # Increase top space to ensure plots start lower
    plt.subplots_adjust(hspace=0.5, top=0.6)

    # Save
    plt.savefig(output_filename)
    plt.close(fig)

    print(f"Combined plot saved to: {output_filename}")


if __name__ == "__main__":
    if len(sys.argv) < 5:
        print("Usage: python plot_lut_stats.py <build_results.csv> <query_results.csv> <counter.csv> <top_image_path>")
        sys.exit(1)
        
    plot_all(sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4])
