# Usage

Running `rq` requires a **JSONPath query**, and a valid **JSON input**.
The query is always provided inline, while the input can come from a file,
standard input, or an argument.

## Input mode

The `rq` app supports three different input sources.

### Input from file

The primary input mode is from a JSON file specified as the second positional
argument. For example, if there's a file in the current directory called
`ex.json` with the contents:

```json
{{#include usage.in/ex.json}}
```

then we can run the query by specifying `./ex.json` as the file path:

```console
$ rq '$..[*].key' ./ex.json
"key1"
"key2"

```

### Inline input

JSON can be passed directly with the `--json` argument:

```console
$ rq '$..*' --json '{ "a": 42, "b": "val" }'
42
"val"

```

This is sometimes more ergonomic when the document is very small.

### Input from stdin

If an input is not provided with other means, `rq` reads from standard input.

**Note**: if the input is a file, it is always more efficient
to provide it as a path than to pipe it to `rq`'s standard input.
Doing `cat $file | rq $query` is an antipattern.

## Output mode

By default `rq` outputs all matched values, in the order they occur in the
document. Of note is that the original formatting is preserved.
For example, if `pretty.json` contains:

```json
{{#include usage.in/pretty.json}}
```

then extracting the nested object will result in:

```console
$ rq '$.key' ./pretty.json
{
        "contents": 0
    }

```

You can see all the original whitespace preserved.[^whitespace-note]

### Count result mode

Sometimes the concrete matches are not interesting, and we only want to count
how many matches there are. This can be done much more efficiently than full
matches, and can be enabled by passing `count` to the `--result` flag
(or its `-r` shorthand).

```console
$ rq '$[*]' --json '[0,1,2,3]' -r count
4
```

### Indices result mode

There is also a result mode that outputs the byte offset in the input document.
This is sometimes useful when you have access to the file and want to perform
post-query custom parsing on the values by correlating the indices with the
original file.

```console
$ rq '$[*]' --json '[0,1,2,3]' -r indices
1
3
5
7

```

## Advanced input options

There are many different ways in which `rq` could read the provided input.
By default it tries its best to decide on the best method.
For example, in file mode it uses memory maps when the files are large.

This might be problematic if memory maps are not available on your machine,
or are very slow for some reason. In that case you can manually override
the input mode with the `--force-input` argument.

The three modes available are:

- `mmap` &ndash; always use memory maps;
- `eager` &ndash; read entire contents of the file or stdin to memory,
run the query after; this makes sense for input documents that are not
excessively large;
- `buffered` &ndash; read the contents in a buffered manner;
this is good for inputs that are very large or have low write
throughput.

[^whitespace-note]: Reformatting the output would decrease performance,
and doing it quickly (for `rsonpath` standards) would take a lot of effort.
It is not impossible, however; if this is a serious issue for your use case, please,
[raise an issue](https://github.com/V0ldek/rsonpath/issues/new?assignees=&labels=type%3A+feature&projects=&template=feature_request.md&title=).
