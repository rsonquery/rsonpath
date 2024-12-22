# Benchmarks for `rsonpath`

Benchmark suite for [`rsonpath`](https://github.com/v0ldek/rsonpath).

| Bench name            | Path                            | Size      | Depth  | Description |
|-----------------------|---------------------------------|-----------|--------|---|
| `ast`                 | `data/ast`                      | -        | - | JSON representation of the AST of an arbitrary popular C file from Software Heritage. To generate the AST `clang` was used: `clang -Xclang -ast-dump=json -fsyntax-only parse_date.c > ast.json` |
| `crossref`            | `data/crossref`                 | -        | - | Concatenation of the first 100 files from [Crossref](https://www.crossref.org/) [source torrent link](https://academictorrents.com/details/e4287cb7619999709f6e9db5c359dda17e93d515)  |
| `openfood`            | `data/openfood`                 | -        | - | Data extracted from [Open Food Facts API](https://wiki.openfoodfacts.org/Open_Food_Facts_Search_API_Version_2) with `curl "https://world.openfoodfacts.org/cgi/search.pl?action=process&tagtype_0=categories&tag_contains_0=contains&tag_0=cheeses&tagtype_1=labels&&json=1" > /tmp/openfood.json` |
| `twitter`             | `data/twitter`                  | -        | -      | Taken from [`simdjson`](https://github.com/simdjson/simdjson) example benchmarks ([permalink](https://github.com/simdjson/simdjson/blob/960a7ebba149af00628e6a56f9605945f91a15b7/jsonexamples/twitter.json)) |
| `wikidata`            | `data/wikidata`                 | -        | - | Arbitrarily chosen datasets from [Wikidata](https://www.wikidata.org/wiki/Wikidata:Data_access) |

## Prerequisites

By default, the benches are performed against the local version of `rsonpath` in the local repository.

1. JDK of version at least 8 is required and your `JAVA_HOME` environment variable must be set
   to its location.
2. You need `gradle` to create the required wrapper JAR to build JSurfer. To install `gradle` it's recommended to use
[`SDKMan`](https://sdkman.io/): `sdk install gradle 7.5`. After that is done, run `just init` in the root of `rsonpath-benchmarks`.
This needs to be done only once.

On x86_64 Ubuntu the latters can be done by installing `openjdk-17-jdk` and exporting `JAVA_HOME` as
`/usr/lib/jvm/java-1.17.0-openjdk-amd64`.

### Download the dataset

Datasets are automatically downloaded on demand when an appropriate benchmark is ran. The datasets are also
automatically checked against their known SHA256 checksum to verify integrity.

## Usage

To benchmark a dataset run

```bash
cargo bench --bench <dataset>
```

You can compare the SIMD and no-SIMD versions by disabling the default `simd` feature:

```bash
cargo bench --bench <dataset> --no-default-features
```

The folder `target/criterion` contains all the information needed to plot the experiment.

## Plotting

To plot the result once the is bench done:

```bash
python3 charts/charts.py
```

You can also provide a path to a `criterion` folder with results:

```bash
python3 charts/charts.py exps/chetemi
```

The plot will be saved in the `plot.png` file of the current directory.

## Statistics

Two statistics scripts are available:

* One about the dataset:

```python
python3 charts/dataset_stat.py
```

It will plot some informations about each JSON file in the `data` folder. Be aware that it will
load the file in memory, in Python. Expect it to be slow and memory consumming.

* One about the queries:

```python
python3 charts/queries_stat.py
```

This script will assume you've run the benchmark to extract the list
of queries from `target/criterion`. It will then compute some parameters and the number of query results with `rsonpath`.
The binary of `rsonpath` should be in the path (run `cargo install rsonpath`).
