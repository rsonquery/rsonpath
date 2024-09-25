import sys
import os
import pandas as pd

import src.plot_heap_size as heap

if __name__ == "__main__":
    # Load the CSV file
    file_path = sys.argv[1]
    directory, filename = os.path.split(file_path)
    file_base_name = os.path.splitext(filename)[0]

    # Plot
    df = pd.read_csv(sys.argv[1])
    heap.plot_size(df, os.path.join(directory, f"{file_base_name}_heap_size.png"))
