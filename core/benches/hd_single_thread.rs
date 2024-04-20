use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
};

pub fn bench(c: &mut Criterion) {
    let mut step = 0;
    c.bench_function("1920x1080 single thread", |b| b.iter(|| {
        let shufmask = black_box(unsafe {
            core::arch::x86_64::_mm256_set_epi64x(
                0x0303030303030303,
                0x0202020202020202,
                0x0101010101010101,
                0x0000000000000000,
            )
        });

        step += 1;
    }));
}

criterion_group!(benches, bench);
criterion_main!(benches);
