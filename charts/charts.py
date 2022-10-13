import os
import pathlib
import json
import sys
import pandas as pd

rootpath = pathlib.Path(__file__).parent.parent
print(rootpath)

def collect_exps(path=None):
    path = pathlib.Path(rootpath, "target", "criterion") if path is None else path
    L = list(os.walk(path))
    L = list(filter(lambda e:"benchmark.json" in e[2] and "new" in e[0], L))
    exps = []
    for upath, _, docs in L:
        p = pathlib.Path(upath, "benchmark.json")
        with open(p) as f:
            d = json.load(f)
            exps.append(d)
        p = pathlib.Path(upath, "estimates.json")
        with open(p) as f:
            t = json.load(f)
            d["estimates"] = {
            "mean": [
                t["mean"]["point_estimate"],
                t["mean"]["standard_error"]
            ],
            "median": [
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
    d = get_exp_data(path=path)
    d2 = {}
    for e,v in d.items():
        if "rsonpath" not in v:
            continue
        d2[e] = h = {}
        for x in v:
            h[x] = v[x]["throughput"]["BytesDecimal"]/v[x]["estimates"]["median"][0]

    df = pd.DataFrame(d2).transpose()
    df = df.sort_index()
    ax = df.plot(kind="barh", ylabel="GB/s", rot=27)
    fig = ax.get_figure()
    fig.set_figwidth(18)
    fig.set_figheight(10)
    fig.savefig("plot.png")
    queries = {}
    for e,v in d.items():
        if "rsonpath" not in v:
            continue
        queries[e] = h = {}
        for x in v:
            h[x] = v[x]["value_str"]
    
    for x,v in sorted(queries.items(), key=lambda e:e[0]):
        print(x)
        for e, q in v.items():
            print(f"\t{e}:{q}")
