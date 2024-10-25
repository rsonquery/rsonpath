from pathlib import Path
from shutil import rmtree
from .draw_plot import generate_graphs_exp
from argparse import ArgumentParser

parser = ArgumentParser(
    prog="A small cli to generate graph based on rsonpath-benchmark",
    description="""
First generate criterion data with rsonpath-benchmark.
The datafolder can be found in target/criterion.""")

parser.add_argument("path",
    help="Path toward the criterion folder",
    type=Path)

parser.add_argument("-o", "--output",
    help="Path where to store the results. By default append _out to the input path",
    type=Path,
    default=None)

parser.add_argument("-e", "--erase",
    help="Flag to allow erasing the output folder if already exists. Default is False",
    action="store_true")

if __name__ == "__main__":
    args = parser.parse_args()
    path = args.path
    output = args.output
    if not output:
        output = Path(path.parent, path.name+"_out")
    if output.exists():
        if not args.erase:
            raise ValueError("directory already exists, erase with -e flags if needed")
        else:
            rmtree(output)
    output.mkdir()

    generate_graphs_exp(str(path), str(output))