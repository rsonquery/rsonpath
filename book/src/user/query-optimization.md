# Query optimization

Our tool's ambition is to be the fastest JSONPath engine of all.
This is done in a similar vain to regex engines, where we try to find
the computationally simplest way of performing a given query. This is
highly dependent on the query itself, and thus it's possible to compromise
performance by making the query less friendly to the engine.

It's not always obvious if a query is going to be "nice" to `rsonpath`.
In this chapter we try to outline some common ways of making a query
faster by rewriting it to a different, yet equivalent, form.

## Starting with a descendant search

The operation that `rsonpath` perform the fastest is looking for the first key
when the query starts with a descendant name selector.

There is no way to make use of this automatically, but as a user you might
have insight into the schema of the documents that are being queried.
Imagine you have a query `$.products[*].videoChapters` that selects
all video chapters from a list of products. It just so happens that
in the input document the _only_ occurrences of "videoChapters"
are within the "products" list. Therefore, a query `..videoChapters`
would be equivalent and select exactly the same nodes.

The above example is an actual real-life case. The rewritten query
is over ten times faster than the original, so an order of magnitude.

Note that this specifically relates only to the _first_ selector being
a descendant selector.

## Omitting wildcards

The wildcard selector is relatively expensive, as it forces the engine
to closely look at every value it encounters. Using reasoning similar
to the one in the previous section it's sometimes possible to eliminate
a wildcard selector by either using a specific name to match, or replacing
it with a descendant selector.

Take an extended query from the above example that digs into structure
to select the "chapter" key of a video chapter: `..videoChapters[*].chapter`.
Again, it just so happens that the query `..videoChapters..chapter` is equivalent,
as all "chapter" keys always occur only ones underneath a "videoChapters" entry.
The rewritten query will be faster

Just as well, it is always better to make the query more specific, if possible.
The query `$['key']<rest>` will always be faster than `$[*]<rest>`.

## Avoiding descendant wildcards

The absolute worst query to run is `$..*`. It requires the engine to look
at every value in the document, nullifying most optimizations.
When facing performance problems, try to express your query without a descendant
wildcard, if possible, or at least to restrict it to a smaller portion of the
document. For example, `$.key..*` will be faster than `$..*` by itself.

## Reporting an issue

We consider performance a paramount feature of `rsonpath`.
If you're facing queries that are excessively slow for your taste,
complain to us by reporting [Issues](issues.md) so that we can benchmark
against your use case.
