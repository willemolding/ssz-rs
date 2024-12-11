use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ssz_rs::prelude::*;

/// Create a multiproof for every element in a list
pub fn large_list_multiproof(c: &mut Criterion) {
    const LIST_SIZE: usize = 10_000;

    let list = List::<Node, LIST_SIZE>::default();
    let element_gindices = (0..LIST_SIZE)
        .map(|i| List::<Node, LIST_SIZE>::generalized_index(&[i.into()]).unwrap())
        .collect::<Vec<_>>();

    c.bench_function("multiproof list", |b| {
        b.iter(|| list.multi_prove_gindices(&element_gindices))
    });
}

criterion_group!(benches, large_list_multiproof);
criterion_main!(benches);
