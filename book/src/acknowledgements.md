# Acknowledgements

The `rsonpath` project was inspired by theoretical work by
Corentin Barloy, Filip Murlak, and Charles Paperman in
[Stackless Processing of Streamed Trees](https://doi.org/10.1145/3452021.3458320).

It would not be possible to create this without prior research into
SIMD-accelerated JSON processing, first by Geoff Langdale and Daniel Lemire
in [Parsing gigabytes of JSON per second](https://doi.org/10.1007/s00778-019-00578-5)
and the [`simdjson` project](https://github.com/simdjson/simdjson), then by
Lin Jiang and Zhijia Zhao in
[JSONSki: streaming semi-structured data with bit-parallel fast-forwarding](https://doi.org/10.1145/3503222.3507719)
and the [`JSONSki` project](https://github.com/automatalab/jsonski).

All references and citations can be found in my master's thesis,
[Fast execution of JSONPath queries](https://v0ldek.com/masters), and in the subsequent paper,
[Supporting Descendants in SIMD-Accelerated JSONPath](https://v0ldek.com/rsonpath-paper).
Both are also hosted in this repository in [/pdf](https://github.com/V0ldek/rsonpath/tree/main/pdf).

Special thanks to Filip Murlak and Charles Paperman for advising
me during my thesis, when most of the fundamentals of the project were born.
