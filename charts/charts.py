import os
import pathlib
import json
import sys
import matplotlib
import matplotlib.pyplot as plot
import numpy as np

print(matplotlib.__version__)
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
            size = v[x]["throughput"]["BytesDecimal"]
            stdev = v[x]["estimates"]["median"][1]
            median = v[x]["estimates"]["median"][0]
            h[x] = size/median #(size/(median+stdev), size/median, size/(median-stdev))
    exps = list(sorted(d2))
    exps_short = [f"Q{i}" for i in range(len(exps))]

    jsurfer = np.array([d2[e].get("jsurfer", 0) for e in exps])
    rsonpath = np.array([d2[e].get("rsonpath", 0) for e in exps])
    jsonski = np.array([d2[e].get("jsonski", 0) for e in exps])

    pos = np.array(range(len(exps)))
    fig, (ax0, ax1) = plot.subplots(1, 2, gridspec_kw={'width_ratios':[1, 2.5]})
    bar = ax0.bar(exps_short, jsurfer, label="jsurfer")
    ax0.set_title("jsurfer")
    ax0.set_ylabel("GB/s")
    ax0.bar_label(bar, [f"{e:0.2f}" for e in jsurfer])

    bar = ax1.bar(pos, rsonpath/jsurfer, label="rsonpath", tick_label=exps_short)
    ax1.bar_label(bar, [f"{e:0.1f}" for e in rsonpath])
    jsonski2 = jsonski/jsurfer
    pos2, jsonski2 = zip(*filter(lambda e:e[1] > 0, zip(pos, jsonski2)))
    jsonski2 = np.array(jsonski2)
    pos2 = np.array(pos2)

    bar = ax1.bar(pos2+0.44, jsonski2, label="jsonski")
    ax1.bar_label(bar, [f"{e:0.1f}" for e in filter(bool, jsonski)])
    ax1.set_title("rsonpath vs jsonski")
    ax1.set_ylabel("Jsurfer ratio")
    ax1.yaxis.set_label_position("right")
    ax1.yaxis.tick_right()
    ax1.legend()
    fig.tight_layout()
    fig.set_size_inches(15, 5)
    plot.savefig("plot.png")

    queries = {}
    for e,v in d.items():
        if "rsonpath" not in v:
            continue
        queries[e] = h = {}
        for x in v:
            h[x] = v[x]["value_str"]

    for i in range(len(exps)):
        print(f"Q{i}: {queries[x]}")
    sys.exit(0)
    for x,v in queries.items():
        print(x)
        for e, q in v.items():
            print(f"\t{e}:{q}")
