import json
import os
import pathlib
from extract_info import *

def depth(tree):
    if type(tree) not in (dict, list):
        return 1
    L = tree
    if type(tree) == dict:
        L = tree.values()
    t = tuple(depth(e) for e in L)
    if t: 
        return max(t) + 1
    else:
        return 1

def density(tree):
    if type(tree) not in (dict, list):
        return 1
    L = tree
    if type(tree) == dict:
        L = tree.values()
    return sum(density(e) for e in L) + 1
    
if __name__ == "__main__":
    print("file", "size", "depth", "verbosity", sep="\t")
    dataset = {}
    for p in get_dataset(): 
        with open(p) as f:
            x = f.read()
        d = json.loads(x)
        size = len(x)
        if size < 1000000:
            size = f"{size/1000:0.1f} KB"
        else:
            size = f"{size/1000000:0.1f} MB"
        print(p.name[:-5], size, depth(d), f"{1/(density(d)/len(x)):0.1f}", sep="\t")
    
    
