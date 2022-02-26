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

| Bench                  | Description                                          | time (ns) |
|------------------------|------------------------------------------------------|-----------|
| **mega_match first**   | first entry of the table                             |    1.9123 |
| **mega_match middle**  | entry in the middle of the table                     |    30.142 |
| **mega_match last**    | last entry in the table                              |    2.9138 |
| **mega_match missing** | entry not in the table (default branch of the match) |    3.1926 |
| **phf first**          | first entry of the table                             |    14.846 |
| **phf middle**         | entry in the middle of the table                     |    14.809 |
| **phf last**           | last entry in the table                              |    17.947 |
| **phf missing**        | entry not in the table                               |    17.422 |

