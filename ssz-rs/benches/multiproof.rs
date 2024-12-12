use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use ssz_rs::prelude::*;

/// Create a multiproof for every element in a list
pub fn large_list_multiproof<const LIST_SIZE: usize>(n_proofs: usize) {
    let list = List::<Node, LIST_SIZE>::default();
    let element_gindices = (0..n_proofs)
        .map(|i| List::<Node, LIST_SIZE>::generalized_index(&[i.into()]).unwrap())
        .collect::<Vec<_>>();
    list.multi_prove_gindices(&element_gindices);
}

/// Create a multiproof for every element in a list
pub fn large_list_multiproof_new<const LIST_SIZE: usize>(n_proofs: usize) {
    let list = List::<Node, LIST_SIZE>::default();
    let element_gindices = (0..n_proofs)
        .map(|i| List::<Node, LIST_SIZE>::generalized_index(&[i.into()]).unwrap())
        .collect::<Vec<_>>();
    list.multi_prove_gindices_new(&element_gindices);
}

fn bench_multiproving(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_list_multiproof");

    const LIST_SIZE: usize = 2_usize.pow(16);

    for n_proofs in [100, 1000, 10000].iter() {
        group.bench_with_input(
            BenchmarkId::new("original", n_proofs),
            &LIST_SIZE,
            |b, n_proofs| b.iter(|| large_list_multiproof::<LIST_SIZE>(*n_proofs)),
        );
        group.bench_with_input(BenchmarkId::new("updated", n_proofs), &LIST_SIZE, |b, n_proofs| {
            b.iter(|| large_list_multiproof_new::<LIST_SIZE>(*n_proofs))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_multiproving);
criterion_main!(benches);
