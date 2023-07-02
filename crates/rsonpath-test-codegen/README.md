# Declarative TOML-driven test case generation for `rsonpath`

The whole pitch of this framework is that we can declare full query engine test documents and queries by writing a straightforward TOML config file:

```toml
[input]
description = "short json with objects and lists, given as an example on jsonpath com"
is_compressed = false

[input.source]
json_string = '''
{
    "firstName": "John",
    "lastName": "doe",
    "age": 26,
    "address": {
        "streetAddress": "naist street",
        "city": "Nara",
        "postalCode": "630-0192"
    },
    "phoneNumbers": [
        {
            "type": "iPhone",
            "number": "0123-4567-8888"
        },
        {
            "type": "home",
            "number": "0123-4567-8910"
        }
    ]
}
'''

[[queries]]
description = "select exact path with name and index selectors"
query = "$.phoneNumbers[0].type"

[queries.results]
count = 1
bytes = [239]
nodes = ['"iPhone"']

[[queries]]
description = "descendant search for 'number'"
query = "$..number"

[queries.results]
count = 2
bytes = [271, 359]
nodes = ['"0123-4567-8888"', '"0123-4567-8910"']

[[queries]]
description = "select first number directly"
query = "$.phoneNumbers[0]"

[queries.results]
count = 1
bytes = [217]
nodes = ['''{
            "type": "iPhone",
            "number": "0123-4567-8888"
        }''']

```

## How it works

This library provides a single public entry point, `generate_tests`, that takes a path to a directory with the source TOML configuration
and a path to an auxillary JSON output directory.

1. It reads all the TOML files and parses them into the structures declared in `model`.
2. It creates compressed versions of all documents that are declared as non-compressed (`input.is_compressed` is false)
using the `compression` module. This will create a new TOML file with the same name, located in the `compressed` subdirectory,
whose input source was processed to be a minified JSON.
3. Each TOML document that has an inline `input.source` has a JSON file created with the given input.
There are two reasons for this: one, testing `MmapInput` requires an actual file; two, putting those strings inline
into the generated test `.rs` file makes it excessively large.
4. Each document gets test cases generated for each declared query, using both engines, all declared result types,
and all supported input modes. This is driven by `gen`.

## Large inputs

The `input.source` property can either contain an inline JSON string as the `json_string` property,
or a relative path to the file containing the actual JSON in the `large_file` property. This is used
for the twitter and wikidata files, since they take multiple megabytes and make the TOML file hard to edit.
