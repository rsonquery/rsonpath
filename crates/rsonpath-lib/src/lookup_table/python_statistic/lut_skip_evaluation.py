import os
import sys
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

def plot(df: pd.DataFrame, file_path: str):
    # Sort by T_ORIGINAL_SKIP for the first plot
    df_sorted = df.sort_values(by="T_ORIGINAL_SKIP")
    
    fig, axes = plt.subplots(2, 2, figsize=(20, 12))
    
    # First plot: T_ORIGINAL, T_LUT, T_OPTIMUM (bar chart)
    df_sorted.plot(x="TEST", y=["T_ORIGINAL", "T_LUT", "T_OPTIMUM"], kind="bar", ax=axes[0, 0])
    axes[0, 0].set_ylabel("Time (ns)")
    axes[0, 0].set_title("Query Execution Time Comparison")
    axes[0, 0].tick_params(axis='x', rotation=45)
    
    # Second plot: T_ORIGINAL_SKIP vs. T_LUT_SKIP
    df_sorted.plot(x="TEST", y=["T_ORIGINAL_SKIP", "T_LUT_SKIP"], kind="bar", ax=axes[0, 1])
    axes[0, 1].set_ylabel("Time (ns)")
    axes[0, 1].set_title("Skip Time Comparison")
    axes[0, 1].tick_params(axis='x', rotation=45)
    
    # Third plot: LUT Build Time per JSON file
    df_unique_json = df.drop_duplicates(subset=["JSON"])
    sns.barplot(data=df_unique_json, x="JSON", y="T_LUT_BUILD", ax=axes[1, 0])
    axes[1, 0].set_ylabel("LUT Build Time (ns)")
    axes[1, 0].set_title("LUT Build Time per JSON File")
    axes[1, 0].tick_params(axis='x', rotation=45)
    
    # Fourth plot: LUT Capacity per JSON file
    sns.barplot(data=df_unique_json, x="JSON", y="LUT_CAPACITY", ax=axes[1, 1])
    axes[1, 1].set_ylabel("LUT Capacity (bytes)")
    axes[1, 1].set_title("LUT Capacity per JSON File")
    axes[1, 1].tick_params(axis='x', rotation=45)
    
    plt.tight_layout()
    plt.savefig(file_path.replace(".csv", "_combined_plots.png"))
    plt.close()

if __name__ == "__main__":
    # Load the CSV file
    file_path = sys.argv[1]
    df = pd.read_csv(file_path)
    
    # Generate the plots
    plot(df, file_path)
