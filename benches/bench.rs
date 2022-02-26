use benchmark::{mega_match, phf};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn mega_match_bench(c: &mut Criterion) {
    // first item in the match
    c.bench_function("mega_match best", |b| {
        b.iter(|| mega_match(black_box("AD")))
    });

    // middle item in the match
    c.bench_function("mega_match avg", |b| b.iter(|| mega_match(black_box("PK"))));

    // last item in the match
    c.bench_function("mega_match worst", |b| {
        b.iter(|| mega_match(black_box("J07BX03")))
    });

    // default branch in the match
    c.bench_function("mega_match default", |b| {
        b.iter(|| mega_match(black_box("THIS IS NOT A VALID ITEM")))
    });
}

pub fn phf_bench(c: &mut Criterion) {
    // first item in the match
    c.bench_function("phf best", |b| b.iter(|| phf(black_box("AD"))));

    // middle item in the match
    c.bench_function("phf avg", |b| b.iter(|| phf(black_box("PK"))));

    // last item in the match
    c.bench_function("phf worst", |b| b.iter(|| phf(black_box("J07BX03"))));

    // default branch in the match
    c.bench_function("phf default", |b| {
        b.iter(|| phf(black_box("THIS IS NOT A VALID ITEM")))
    });
}

criterion_group!(benches, mega_match_bench, phf_bench);
criterion_main!(benches);
