use benchmark::{mega_match, phf};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn mega_match_bench(c: &mut Criterion) {
    // first item in the match
    c.bench_function("mega_match first", |b| {
        b.iter(|| mega_match(black_box("AD")))
    });

    // middle item in the match
    c.bench_function("mega_match middle", |b| {
        b.iter(|| mega_match(black_box("PK")))
    });

    // last item in the match
    c.bench_function("mega_match last", |b| {
        b.iter(|| mega_match(black_box("J07BX03")))
    });

    // default branch in the match
    c.bench_function("mega_match missing", |b| {
        b.iter(|| mega_match(black_box("THIS IS NOT A VALID ITEM")))
    });
}

pub fn phf_bench(c: &mut Criterion) {
    // first item in the match
    c.bench_function("phf first", |b| b.iter(|| phf(black_box("AD"))));

    // middle item in the match
    c.bench_function("phf middle", |b| b.iter(|| phf(black_box("PK"))));

    // last item in the match
    c.bench_function("phf last", |b| b.iter(|| phf(black_box("J07BX03"))));

    // default branch in the match
    c.bench_function("phf missing", |b| {
        b.iter(|| phf(black_box("THIS IS NOT A VALID ITEM")))
    });
}

criterion_group!(benches, mega_match_bench, phf_bench);
criterion_main!(benches);
