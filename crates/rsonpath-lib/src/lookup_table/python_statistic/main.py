import sys
import os
import pandas as pd

import src.plot_stats as stats
import src.plot_vs as vs

if __name__ == "__main__":
    # Check if there are any command-line arguments passed
    if len(sys.argv) > 1:
        # Load the CSV file
        file_path = sys.argv[1]
        df_stats = pd.read_csv(file_path)
        directory, file_name = os.path.split(file_path)
        file_base_name = os.path.splitext(file_name)[0]

        # Plot naive stats
        stats.plot_time(df_stats, os.path.join(directory, f"{file_base_name}_stats_time.png"))
        stats.plot_size(df_stats, os.path.join(directory, f"{file_base_name}_stats_size.png"))
        stats.plot_speed(df_stats, os.path.join(directory, f"{file_base_name}_stats_speed.png"))
        
        # Plot naive vs distance
        df_vs = pd.read_csv(sys.argv[2])
        vs.plot_size(df_vs, os.path.join(directory, f"{file_base_name}_vs_size.png"))
        vs.plot_speed(df_vs, os.path.join(directory, f"{file_base_name}_vs_speed.png"))
    else:
        print("No parameters were passed. Please provide the path to the CSV file.")
