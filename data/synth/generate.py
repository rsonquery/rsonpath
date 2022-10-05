import random, sys, json
label = lambda :random.choice("aab")
branch = lambda :random.choice(L)
def generate(depth):
    if not depth:
        return "l"
    b = random.choice([1, 1, 1, 1, 1, 1, 2])
    if b == 1:
        d = {label():generate(depth-1)}
    else:
        d = {"a": generate(depth-1), "b":generate(depth-1)}
    return d

def json_generate(depth):
    return json.dumps(generate(depth))

seed = 124
depth = 75

if len(sys.argv)>1:
    depth = int(sys.argv[1])
    if len(sys.argv) > 2:
        seed = int(sys.argv[2])
random.seed(seed)
print(json_generate(depth))
