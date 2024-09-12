import sys
import os
import pandas as pd

import src.plot_compare as compare

if __name__ == "__main__":
    # Load the CSV file
    file_path = sys.argv[1]
    directory, filename = os.path.split(file_path)
    file_base_name = os.path.splitext(filename)[0]

    # Plot
    df = pd.read_csv(sys.argv[1])
    compare.plot_size(df, os.path.join(directory, f"{file_base_name}_vs_size.png"))
    compare.plot_speed(df, os.path.join(directory, f"{file_base_name}_vs_speed.png"))
