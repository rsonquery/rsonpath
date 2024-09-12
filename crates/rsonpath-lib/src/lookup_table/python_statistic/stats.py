import sys
import os
import pandas as pd

import src.plot_stats as stats

if __name__ == "__main__":
    # Load the CSV file
    file_path = sys.argv[1]
    directory, filename = os.path.split(file_path)
    file_base_name = os.path.splitext(filename)[0]

    # Plot
    df = pd.read_csv(file_path)
    stats.plot_time(df, os.path.join(directory, f"{file_base_name}_stats_time.png"))
    stats.plot_size(df, os.path.join(directory, f"{file_base_name}_stats_size.png"))
    stats.plot_speed(df, os.path.join(directory, f"{file_base_name}_stats_speed.png"))

