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

By default, the benches are performed against a released version of `rsonpath`.
Usually you might want to run it against the local version to test your changes.
To do that, pass a [patch config value] to `cargo`:

```ini
--config 'patch.crates-io.rsonpath.path = "../rsonpath"'
```

Additionally:

1. An appropriate C++ compiler is required for the [`cc` crate](https://lib.rs/crates/cc) to compile the
   JSONSki code.
2. JDK of version at least 8 is required and your `JAVA_HOME` environment variable must be set
   to its location.

On x86_64 Ubuntu the latters can be done by installing `openjdk-17-jdk` and exporting `JAVA_HOME` as
`/usr/lib/jvm/java-1.17.0-openjdk-amd64`.

### Download the dataset

On a UNIX system with `wget` installed run the script `sh dl.sh`.
You can also manually download the dataset and put the JSON files in the correct folder.

For more information, refers to:

* AST: [![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.7229269.svg)](https://doi.org/10.5281/zenodo.7229269)
* Twitter: [![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.7229287.svg)](https://doi.org/10.5281/zenodo.7229287)
* Crossref: [![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.7229287.svg)](https://doi.org/10.5281/zenodo.7231920)

For the benchmark to work, the directory layout should be as follows:

```ini
── data
   ├── ast
   │   └── ast.json
   ├── crossref
   │   ├── crossref0.json
   │   ├── crossref16.json
   │   ├── crossref1.json
   │   ├── crossref2.json
   │   ├── crossref4.json
   │   └── crossref8.json
   └── twitter
       └── twitter.json
```

The sha256sum of the JSON files, for reference:

* `c3ff840d153953ee08c1d9622b20f8c1dc367ae2abcb9c85d44100c6209571af  ast/ast.json`
* `f76da4fbd5c18889012ab9bbc222cc439b4b28f458193d297666f56fc69ec500  crossref/crossref/crossref1.json`
* `95e0038e46ce2e94a0f9dde35ec7975280194220878f83436e320881ccd252b4  crossref/crossref/crossref2.json`
* `f14e65d4f8df3c9144748191c1e9d46a030067af86d0cc03cc67f22149143c5d  twitter/twitter.json`

TODO: checksums of other crossrefs

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

As a reminder, to test against local changes instead of a crates.io version:

```bash
cargo bench --bench <dataset> --config 'patch.crates-io.rsonpath.path = "../rsonpath"'
```

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
