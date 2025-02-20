import os
import sys
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

def plot(df: pd.DataFrame, file_path: str):
    # Sort by T_ORIGINAL_SKIP for better visualization
    df_sorted = df.sort_values(by="T_ORIGINAL_SKIP")
    
    # Separate data by COLOR value
    df_blue = df_sorted[df_sorted["COLOR"] == "blue"]
    df_yellow = df_sorted[df_sorted["COLOR"] == "yellow"]
    
    fig, axes = plt.subplots(1, 2, figsize=(20, 6))
    
    for i, (df_subset, color, ax) in enumerate(zip([df_blue, df_yellow], ["Blue", "Yellow"], axes)):
        if not df_subset.empty:
            build_time = df_subset["T_LUT_BUILD"].iloc[0] / 1_000_000  # Convert ns to ms
            capacity = df_subset["LUT_CAPACITY"].iloc[0] / (1024 * 1024)  # Convert bytes to MB
            title = f"Query Execution Time Comparison ({color} Data)\nLUT Build Time: {build_time} ms, LUT Capacity: {capacity:.2f} MB"
            
            df_subset.plot(
                x="TEST", 
                y=["T_ORIGINAL", "T_LUT", "T_OPTIMUM", "T_ORIGINAL_SKIP", "T_LUT_SKIP"], 
                kind="bar", 
                ax=ax,
                color=["salmon", "blue", "green", "lightsalmon", "lightblue"]
            )
            ax.set_ylabel("Time (ns)")
            ax.set_title(title)
            ax.tick_params(axis='x', rotation=45)
    
    plt.tight_layout()
    plt.savefig(file_path.replace(".csv", "_bar_plots.png"))
    plt.close()

if __name__ == "__main__":
    # Load the CSV file
    file_path = sys.argv[1]
    df = pd.read_csv(file_path)
    
    # Generate the plots
    plot(df, file_path)
