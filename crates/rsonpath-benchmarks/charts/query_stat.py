import json
import os
import pathlib
from extract_info import *
import subprocess
import sys
path = None
if len(sys.argv) > 1:
    path = pathlib.Path(sys.argv[1])
    if not path.is_dir():
        raise ValueError("Expect a path to a directory in input")

exp_data = get_exp_data(path=path)
short_exps, exps = get_query_names(path=path)
datasets = {e.parent.name:e for e in get_dataset(path=path)}
queries = []
for i in range(len(exps)):
    queries.append((short_exps[i], exp_data[exps[i]]["rsonpath"]["value_str"], exps[i].split("_")[0]))
print("\n".join(map(str, queries)))
binary = pathlib.Path(rootpath.parent, "rsonpath", "target", "release", "rsonpath")
print(binary)
print("short name", "match", "query", sep="&\t", end="\\\\\n")
print("\\hline")
for t in queries:
    p = subprocess.Popen([str(binary), "-r", "count", str(t[1]), str(datasets[t[2]])], stdout=subprocess.PIPE)
    query = "\\texttt{"+t[1].replace("$", "\\$").replace("_","\_")+"}"
    print(t[0], query, p.stdout.read().decode().strip(), sep="&\t", end="\\\\\n")
