import sys
import os
import pandas as pd

import src.plot_count_distances as cd

if __name__ == "__main__":
    # Load the CSV file
    file_path = sys.argv[1]
    directory, filename = os.path.split(file_path)
    file_base_name = os.path.splitext(filename)[0]

    # Plot
    df = pd.read_csv(file_path)
    cd.plot_binned_frequencies(df, os.path.join(directory, f"{file_base_name}_plot.png"))

