# mega-match-vs-phf

Benchmark to compare performance of a big (100s items) match statement against Perfect Hash Function (phf).


## Context

In the context of the [`dgc`](https://github.com/rust-italia/dgc) library, we have a helper function that updates values against a big (~300 items) lookup table.

The lookup table [is implemented statically through a big match statement](https://github.com/rust-italia/dgc/blob/main/dgc/src/valuesets.rs#L36).

I was suggested to look at [`phf`](https://docs.rs/phf/latest/phf/) as possible alternative and that it could provide a faster implementation.

This repository tries to assess, for the specific use case described here, whether `phf` could result in better performance.

Keep in mind that this is driven by pure curiosity. The functionality illustrated here is a secondary piece of functionality and it is rarely used by the users. The complexity of introducing an additional dependency is not expected to justify some minor performance gain here.


## Benchmark

To run the benchmarks locally:

```bash
cargo bench
```

You will see the results in the stdout or, for an HTML report, check out `target/criterion/report/index.html`.


## Current results

Run locally on a MacBook Pro (13-inch, M1, 2020)  Apple M1 - 16 Gb

Rust version: `rustc 1.59.0 (9d1b2106e 2022-02-23)`

| Item position | `mega_match` | `phf`      |
|---------------|-------------:|-----------:|
| first         | **1.877**    | 14.363     |
| middle        | 29.384       | **14.431** |
| last          | **2.837**    | 17.535     |
| missing       | **6.886**    | 13.055     |

Times are in nanoseconds (ns).


## Conclusions

The mega match is obviously way faster when trying to fech the first item, but it's slower than `phf` when looking up an item in the middle of the mega match.

What's interesting is that the mega match seems to be very very fast (and way faster than `phf`) also when looking up items at the very end of the match (or even missing items).

~~It's currently not clear why this happens but there's probably some smart compilation happening here and it could be worth to look at the generated assembly.~~

I had a look at the generated assembly for [a simplified case similar to this one](https://rust.godbolt.org/z/1vdq8YTc3) and it seems the compiler is NOT generating any smart data structure (e.g. a prefix tree) to speed up this particular case.

What's more likely to happen here is that branch prediction makes this code very fast in certain circumstances.

Feel free to contribute to this project if you know what's really going on here.

Finally, it's worth noting that `phf` has a pretty constant run time (only sliightly variable by the length of the input data) which is not dependent from the position of the item in the lookup table.


## Contributing

Everyone is very welcome to contribute to this project.
You can contribute just by submitting bugs or suggesting improvements by
[opening an issue on GitHub](https://github.com/lmammino/mega-match-vs-phf/issues).


## License

Licensed under [MIT License](LICENSE). © Luciano Mammino.