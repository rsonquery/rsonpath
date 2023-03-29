# How to contribute

- Found a bug? Check if it was reported yet as a
[![GitHub issues by-label](https://img.shields.io/github/issues/v0ldek/rsonpath/type:%20bug?color=%23d73a4a&label=bug&logo=github)](https://github.com/V0ldek/rsonpath/labels/type%3A%20bug),
if not then file a new [bug issue](https://github.com/V0ldek/rsonpath/issues/new?labels=type%3A+bug&template=bug_report.md)
- Want an enchancement? Check if it's already proposed as a
[![feature issues](https://img.shields.io/github/issues/v0ldek/rsonpath/type:%20feature?color=%23b2feff&label=feature&logo=github)](https://github.com/V0ldek/rsonpath/labels/type%3A%20feature)
, if not then file a new [feature issue](https://github.com/V0ldek/rsonpath/issues/new?labels=type%253A+feature&template=feature_request.md)

Every newly created issue gets assigned the
[![triage issues](https://img.shields.io/github/issues/v0ldek/rsonpath/acceptance:%20triage?color=%2384A6B5&label=acceptance%3A%20triage&logo=github)](https://github.com/V0ldek/rsonpath/labels/acceptance%3A%20triage)
label. Once reviewed,
it will be assigned a milestone and the label will be exchanged for
[![go ahead issues](https://img.shields.io/github/issues/v0ldek/rsonpath/acceptance:%20go%20ahead?color=%23FF4400&label=acceptance%3A%20go%20ahead&logo=github)](https://github.com/V0ldek/rsonpath/labels/acceptance%3A%20go%20ahead)
to signal it is of value to the project and
can be worked on (for a feature), or that it is indeed a bug that we need to fix (for a bug).

## Not sure?

Go to [Discussions](https://github.com/V0ldek/rsonpath/discussions), where you can drop more open-ended questions and ideas without having to formulate them as detailed Issues!

## Code contributions

You want to write code for the crate? Great! First, you need an issue to contribute to,
one marked as
[![go ahead issues](https://img.shields.io/github/issues/v0ldek/rsonpath/acceptance:%20go%20ahead?color=%23FF4400&label=acceptance%3A%20go%20ahead&logo=github)](https://github.com/V0ldek/rsonpath/labels/acceptance%3A%20go%20ahead).
You can use
[![help wanted issues](https://img.shields.io/github/issues/v0ldek/rsonpath/contribute:%20help%20wanted?color=%23008672&label=contribute%3A%20help%20wanted&logo=github)](https://github.com/V0ldek/rsonpath/labels/contribute%3A%20help%20wanted),
meaning "I'd be very happy if someone implemented this",
or
[![good first issue issues](https://img.shields.io/github/issues/v0ldek/rsonpath/contribute:%20good%20first%20issue?color=%23008672&label=contribute%3A%20good%20first%20issue&logo=github)](https://github.com/V0ldek/rsonpath/labels/contribute%3A%20good%20first%20issue),
meaning "I'd be happy if someone implemented this and it's relatively straightforward".
Go to the issue and post a comment that you're going to work on it. [Fork the repo](https://github.com/V0ldek/rsonpath/fork),
write your feature of fix, then create a PR.

### Setting up local development

[Fork the repo](https://github.com/V0ldek/rsonpath/fork) and clone the repository locally.

You will need Rust/`cargo` installed and `just`.

To install Rust refer to [rustup.rs](https://rustup.rs/).

[Just](https://github.com/casey/just) is an alternative to `make` and we use it for project-specific commands.
It can be installed through most package managers, including `apt`, `snap`, `brew`, `choco`, and `cargo` itself.
See [here](https://github.com/casey/just#packages) for complete list.

Once you have `just`, **RUN `just init`**! This will install our git hooks and setup submodules in the repo.

Make sure the project builds by running `just build`.

### Cheatsheet

Quick, common commands have a one-letter alias:

- `just b` &ndash; build the binary in debug mode;
- `just r *ARGS` &ndash; run the debug binary with given arguments, e.g. `just r '$.a.b' -v`
- `just v` &ndash; verify that the lib and bin compile (with `cargo check`);
- `just t` &ndash; run the fast unit tests of the library.

#### Tests

Run `just test` to execute all tests in the project. This includes real dataset end-to-end tests,
so might take a minute or so. The `just t` command runs only unit tests, which is very quick,
and `just doctest` runs doctests.

**Note:** this requires your machine to support one of the SIMD implementations (currently only AVX2).
If it doesn't, you won't be able to run the full test suite, and you're forced to use the package with
`--no-default-features`.

#### Benchmarks

You can run all benchmarks with `just bench`.

Consult the README file of the `rsonpath-benchmarks` project in `/rsonpath-benchmarks/README.md`.

#### Linting

To verify your code before pushing, use `just verify`. It will make sure it's formatted correctly,
builds, passes all tests, and doesn't emit warnings. It will also build the documentation.

There are specialised commands for lighter verifications, like `v`, `verify-fmt`, and `verify-clippy`.

### Guidelines

1. Use standard `rustfmt` settings for formatting.
2. Lint your code with [`clippy`](https://github.com/rust-lang/rust-clippy).
3. Follow [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/), more on it below.
4. Avoid adding new dependencies to Cargo.toml unless there is a good reason for them.

### Commit messages

Your commit messages should look like this:

```ini
type: short description of changes

More detailed description of my change.
Potentially multiline.

Refs: #69

# This project follows Conventional Commits 1.0.0 (https://www.conventionalcommits.org/en/v1.0.0/)
#
# The first line should be a short description of the purpose of the commit.
# Allowed types are: build, ci, docs, feat, fix, perf, refactor, style, test, chore
#
###
# Note: If introducing a breaking change, add an exclamation mark after the type
# Example: fix!: removed `AsRef` impl in favour of `relax_alignment`
### 
# The second section contains a detailed description.
# Always give a description for features,
# omit only for real one-liners like dependency bumps.
# Footer are at the end. Most important are refs, 
# which tell us which GitHub issue the change is related to, if any.
###
# Example:
###
# feat: add dot wildcard selector
#
# Refs: #69
```
