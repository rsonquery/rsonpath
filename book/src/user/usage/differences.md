# `rsonpath`-specific behavior

We try to implement the [JSONPath spec](https://datatracker.ietf.org/doc/draft-ietf-jsonpath-base/)
as closely as possible. There are currently two major differences between
`rsonpath`'s JSONPath and the standard.

## Nested descendant segments

The standard semantics of the descendant segment lead to duplicated results,
and a potentially exponential blowup in execution time and output size.
In `rsonpath` we diverge from the spec to guarantee unduplicated results:

```console
$ rq '$..a..a' --json '{"a":{"a":{"a":42}}}'
{"a":42}
42

```

In standard semantics the value `42` would be matched twice[^descendant-note].

## Unicode

Currently `rsonpath` compares JSON keys bytewise, meaning that labels using
Unicode escape sequences will be handled incorrectly.

For example, the key `"a"` can be equivalently represented by
`"\u0041"`.

```console
$ rq '$["a"]' --json '{"a":42}'
42

```

The above results should be the same if either or both of the `"a"` characters
were replaced with the `"\u0041"` unicode escape sequence, but `rsonpath` does
not support it at this time. This limitation is known and tracked at
[#117](https://github.com/v0ldek/rsonpath/issues/117).

---

[^descendant-note]: The reason behind this is a bit subtle. The standard
defines the result as a concatenation of lists of results of executing
the rest of the query after a descendant segment, _and_ a recursive execution
of the entire query. So the inner `"a"` key in the example is matched first
when evaluating the outermost one, and then again when evaluating the middle `"a"`.
We consider this to be counter-intuitive and undesirable.
