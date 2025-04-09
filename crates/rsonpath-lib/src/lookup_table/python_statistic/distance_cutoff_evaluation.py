import pandas as pd
import matplotlib.pyplot as plt
import matplotlib.image as mpimg
import sys
import os

# Custom color palette
PLOT_COLORS = [
    'red', 'skyblue', 'blue', 'orange', 'green',
    'coral', 'purple', 'pink', 'cyan', 'magenta',
    'brown', 'turquoise', 'darkgreen', 'darkred', 'darkcyan',
    'darkorange', 'darkviolet', 'lime', 'indigo', 'gold',
    'darkblue', 'chartreuse', 'teal', 'yellow', 'salmon',
    'darkkhaki', 'crimson', 'peru', 'forestgreen', 'mediumpurple', 'black',
]


def plot_all(build_csv_path: str, query_csv_path: str, counter_csv_path: str, top_image_path):
    # Load CSV files
    query_df = pd.read_csv(query_csv_path)
    build_df = pd.read_csv(build_csv_path)
    counter_df = pd.read_csv(counter_csv_path)

    # Sort counter_df by TOTAL_PERCENT_SKIP & filter relevant query names
    counter_df = counter_df.sort_values("TOTAL_PERCENT_SKIP")
    counter_df = counter_df[counter_df["QUERY_NAME"].isin(
        query_df["QUERY_ID"])]

    # Merge and sort query_df
    query_df = query_df.merge(
        counter_df[['QUERY_NAME', 'TOTAL_PERCENT_SKIP']],
        left_on='QUERY_ID',
        right_on='QUERY_NAME',
        how='left'
    )
    query_df = query_df.sort_values("TOTAL_PERCENT_SKIP")

    # Convert size to MB and sort
    build_df = build_df.sort_values("CUTOFF")
    build_df["SIZE_MB"] = build_df["SIZE_IN_BYTES"] / (1024 * 1024)

    # Output filename
    output_dir = os.path.dirname(os.path.abspath(build_csv_path))
    json_filename = build_df["JSON"].iloc[0]
    output_filename = os.path.join(output_dir, f"{json_filename}.png")

    # Setup figure
    fig = plt.figure(figsize=(16, 14))

    # Load and display image
    if os.path.exists(top_image_path):
        img = mpimg.imread(top_image_path)
        ax_img = fig.add_axes([0, 0.6, 1, 0.4])
        ax_img.imshow(img)
        ax_img.axis('off')

    # Create cutoff color map using the custom color palette
    unique_cutoffs = sorted(query_df["CUTOFF"].unique())
    cutoff_color_map = {
        cutoff: PLOT_COLORS[i % len(PLOT_COLORS)]
        for i, cutoff in enumerate(unique_cutoffs)
    }

    # --- Plot 1: Query Time Line Plot ---
    ax1 = fig.add_subplot(2, 2, 1)
    for i, (cutoff, group) in enumerate(query_df.groupby("CUTOFF")):
        ax1.plot(
            group["QUERY_ID"].astype(str),
            group["QUERY_TIME_SECONDS"],
            marker='o',
            label=f"Cutoff {cutoff}",
            color=cutoff_color_map[cutoff],
            alpha=0.4
        )
    ax1.set_xlabel("Query ID")
    ax1.set_ylabel("Query Time (seconds)")
    ax1.set_title("Query Time by Query ID (Grouped by Cutoff)")
    ax1.grid(True)
    ax1.set_xticklabels(group["QUERY_ID"].astype(str), rotation=90, fontsize=9)

    # --- Plot 2: Build Time ---
    ax2 = fig.add_subplot(2, 2, 2)
    ax2.bar(
        build_df["CUTOFF"].astype(str),
        build_df["BUILD_TIME_SECONDS"],
        color=[cutoff_color_map[cutoff] for cutoff in build_df["CUTOFF"]]
    )
    ax2.set_xlabel("Cutoff")
    ax2.set_ylabel("Build Time (seconds)")
    ax2.set_title("Build Time by Cutoff")

    # --- Plot 3: Total Percent Skip ---
    ax3 = fig.add_subplot(2, 2, 3)
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

    # --- Plot 4: Size in MB ---
    ax4 = fig.add_subplot(2, 2, 4)
    ax4.bar(
        build_df["CUTOFF"].astype(str),
        build_df["SIZE_MB"],
        color=[cutoff_color_map[cutoff] for cutoff in build_df["CUTOFF"]]
    )
    ax4.set_xlabel("Cutoff")
    ax4.set_ylabel("LUT Size (MB)")
    ax4.set_title("LUT Size by Cutoff")

    # Layout tweaks
    plt.subplots_adjust(hspace=0.5, top=0.6)
    plt.savefig(output_filename)
    plt.close(fig)

    print(f"Combined plot saved to: {output_filename}")


if __name__ == "__main__":
    if len(sys.argv) < 5:
        print("Usage: python plot_lut_stats.py <build_results.csv> <query_results.csv> <counter.csv> <top_image_path>")
        sys.exit(1)

    plot_all(sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4])
