import os
import pathlib
import json
import sys
import matplotlib
import texttable
import numpy as np
import math
import pandas as pd
from extract_info import *
from draw_plot import print_plot as plot



if __name__ == "__main__":
    path = None
    if len(sys.argv) > 1:
        path = pathlib.Path(sys.argv[1])
        if not path.is_dir():
            raise ValueError("Expect a path to a directory in input")
    
    data = get_exp_data(path)
    data = process_exp_data(data)
    benches = sorted(map(format_bench, data))
    filtered_bench = ("included_from", "author_affiliation")
    benches = list(filter(lambda e:"scala" not in e[1] and e[1] not in filtered_bench, benches))
    exps = [f"{e[0]}_{e[1]}" for e in benches]
    exps_short = [f"{e[0].upper()}{i}" for i,e  in enumerate(exps)]
    print("\n".join(f"{e}: {f}" for e,f in zip(exps_short, exps)))
    mapping = {e:data[benches[i][2]] for i,e in enumerate(exps_short)}
    jsurfer = np.array([mapping[e].get("jsurfer", 0) for e in exps_short])
    rsonpath = np.array([mapping[e].get("rsonpath", 0) for e in exps_short])
    jsonski = np.array([mapping[e].get("jsonski", 0) for e in exps_short])
    plot(rsonpath, jsurfer, jsonski, exps_short)
