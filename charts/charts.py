import os
import pathlib
import json
import sys

rootpath = pathlib.Path(__file__).parent.parent

def collect_exps(path=None):
    path = pathlib.Path(rootpath, "target", "criterion") if path is None else path
    L = list(os.walk(path))
    L = list(filter(lambda e:"benchmark.json" in e[2] and "new" in e[0], L))
    exps = []
    for upath, _, docs in L:
        p = pathlib.Path(path, upath, "benchmark.json")
        with open(p) as f:
            d = json.load(f)
            exps.append(d)
        p = pathlib.Path(path, upath, "estimates.json")
        with open(p) as f:
            t = json.load(f)
            d["estimates"] = {
            "mean": [
                t["mean"]["point_estimate"],
                t["mean"]["standard_error"]
            ],
            "medan": [
                t["median"]["point_estimate"],
                t["median"]["standard_error"]
            ]
            }
    return exps

def get_exp_data(path=None):
    exps = collect_exps(path=path)
    groups = {}
    for e in exps:
        fname = e["function_id"]
        if fname not in ("rsonpath", "jsonski", "jsurfer"):
            continue
        groups[e["group_id"]] = L = groups.get(e["group_id"], {})
        L[fname] = e
    return groups

if __name__ == "__main__":
    path = None
    if len(sys.argv) > 1:
        path = pathlib.Path(sys.argv[1])
        if not path.is_dir():
            raise ValueError("Expect a path to a directory in input")
    print(json.dumps(get_exp_data(path=path)))
