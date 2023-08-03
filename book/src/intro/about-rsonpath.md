# What is rsonpath?

The `rsonpath` project comprises two parts:

- the CLI tool `rq` for blazingly fast querying of JSON files from the command line; and
- the underlying `rsonpath-lib` Rust crate allowing one to run structure-aware
JSONPath queries on JSONs as easily as one would query a string with the `regex` crate.

It is both a production-ready app and crate, and a research project,
aiming to be the fastest possible JSONPath implementation in a streaming setting.

It is perfectly suited for command line use, able to handle files that would
not fit into the main memory. Its minimal memory footprint makes the crate
a great choice where one does not want to pay in allocations to extract
data from a JSON payload.

## Why choose `rsonpath`?

If you work with JSONs a lot, a CLI tool for extracting data from your
files or API call responses can be invaluable.

The most popular tool for working with JSONs
is [`jq`](https://jqlang.github.io/jq/), but it has its shortfalls:

- it is _extremely_ slow [^jq-speed-note];
- it has a _massive_ memory overhead [^jq-memory-note];
- its query language is non-standard.

To be clear, `jq` is a great and well-tested tool, and `rq` does not directly
compete with it. If one could describe `jq` as a "`sed` or `awk` for JSON",
then `rq` would be a "`grep` for JSON". It does not allow you to slice and
reorganize JSON data like `jq`, but instead outclasses it on the filtering
and querying applications.

## `rq`

The `rq` CLI app can process JSON documents streamed into stdin or from a file,
outputting query matches to stdout. It has a minimal memory footprint
and processes the input as a stream, maximizing performance.

### When to choose `rq`?

- when you need a general-purpose JSONPath CLI tool; OR
- when working with really big JSON files (gigabytes of size),
where other tools take too long or run out of memory; OR
- when the input is a stream with possibly long delays between chunks,
for example a network socket.

### When does `rq` fall short?

- when Unicode escape sequences are used
([issue #117](https://github.com/V0ldek/rsonpath/issues/117));
- when advanced JSONPath selectors are required
([area: selector issues](https://github.com/V0ldek/rsonpath/issues?q=is%3Aopen+is%3Aissue+label%3A%22area%3A+selector%22));
- when targetting a `no-std` environment [^no-std-note].

## `rsonpath-lib`

The `rsonpath-lib` crate is a JSONPath library serving as the backend of `rq`.
It is a separate product, providing a wider API surface and extensibility
options beyond those of the CLI.

### When to choose `rsonpath-lib`?

- when an application spends a lot of time querying JSON documents
(or parsing those for querying); OR
- the application only needs to parse and create a DOM for parts of the JSON
that are extracted by a query;
- when a minimal memory footprint of JSON processing is desired; OR
- when JSON data comes in a stream (a `Read` impl) and can be queried
in-flight;
- when a tested JSONPath parser is needed for custom JSONPath processing.

### When does `rsonpath-lib` fall short?

- when the entire JSON document needs to be parsed into an in-memory model anyway
for further processing [^dom-note];
- when Unicode escape sequences are used
([issue #117](https://github.com/V0ldek/rsonpath/issues/117));
- when advanced JSONPath selectors are required
([area: selector issues](https://github.com/V0ldek/rsonpath/issues?q=is%3Aopen+is%3Aissue+label%3A%22area%3A+selector%22));
- when targetting a `no-std` environment [^no-std-note].

---

[^jq-speed-note]: Even on queries adversarial to `rq` it can be up to
$40 \times$ faster than `jq`, which takes over a second to process
a $20$ MB file.

[^jq-memory-note]: `jq` can consume upwards of $6.5 \times$ the size of the
JSON document in heap memory. On a $22$ MB file it reaches a peak of $150$ MB.
On the same file and query `rq` uses $12$ KB -- a miniscule fraction of the
file size.

[^no-std-note]: As far as we are aware there are no Rust JSON query
engines that would target `no-std`. It would be possible for
`rsonpath` to require only `alloc` and no `std` -- if this is a feature
you would like to see, please [let us know](https://github.com/V0ldek/rsonpath/issues).

[^dom-note]: Performance gains of `rsonpath` are nullified then, since
there is no benefit of a rapid, low memory query processor if the full document
gets parsed later anyway. In such a case,
[`serde_json_path`](https://lib.rs/crates/serde_json_path) or a different
crate could suit one better. Note that restricting parsing
to fragments of a document returned by a filtering query can still yield
important gains.
