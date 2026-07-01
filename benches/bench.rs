use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

use rsomics_reaching_centrality::global_reaching_centrality_from_edge_list;

/// Deterministic directed Erdős–Rényi G(n, p) edge list via a splitmix64 stream.
fn gnp_edges(n: usize, p: f64, seed: u64) -> String {
    let mut state = seed;
    let mut next = || {
        state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    };
    let mut out = String::new();
    for u in 0..n {
        for v in 0..n {
            if u == v {
                continue;
            }
            let r = (next() >> 11) as f64 / (1u64 << 53) as f64;
            if r < p {
                out.push_str(&format!("{u} {v}\n"));
            }
        }
    }
    out
}

fn bench_global(c: &mut Criterion) {
    let edges = gnp_edges(400, 0.02, 12345);
    c.bench_function("global_gnp400_p0.02_directed", |b| {
        b.iter(|| {
            let g = global_reaching_centrality_from_edge_list(black_box(&edges), black_box(true))
                .unwrap();
            black_box(g)
        })
    });
}

criterion_group!(benches, bench_global);
criterion_main!(benches);
