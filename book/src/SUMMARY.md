# Summary

[What is `rsonpath`?](intro/about-rsonpath.md)
[Who is this book for?](intro/about-this-book.md)

---

# Part I &ndash; CLI User Guide

- [User introduction](user/intro.md)
- [Installation](user/installation.md)
  - [Manual build for maximum performance](user/installation/manual.md)
- [Usage](user/usage.md)
  - [JSONPath reference](user/usage/jsonpath.md)
  - [`rsonpath`-specific behavior](user/usage/differences.md)
- [Query optimization](user/query-optimization.md)
- [Reporting issues](user/issues.md)

# Part II &ndash; Library User Guide

- [Library user introduction](lib/intro.md)
- [Depending on `rsonpath`]()
  - [SIMD support]()
- [API overview]()
  - [Query]()
  - [Input]()
    - [Scenario: JSON in memory]()
    - [Scenario: JSON in a file]()
    - [Scenario: JSON from a stream]()
  - [Sink]()
    - [Scenario: saving to memory]()
    - [Scenario: saving to a stream]()
- [Advanced scenarios]()
  - [Custom `Input` impl]()
  - [Custom `Sink` impl]()

# Part III &ndash; Developer Guide (WIP)

- [Developer introduction](dev/intro.md)

---

[Acknowledgements](acknowledgements.md)
