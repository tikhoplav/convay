use conway::byte_state::{new, random, tick, State};
use criterion::{
    black_box, criterion_group, criterion_main, measurement::Measurement, BenchmarkGroup,
    BenchmarkId, Criterion,
};

fn compare_ticks<const SIZE: usize>(group: &mut BenchmarkGroup<impl Measurement>) {
    // sha256 "Hello, World!!"
    let seed: [u8; 32] = [
        3, 127, 146, 120, 48, 195, 83, 11, 249, 175, 8, 97, 14, 74, 238, 206, 54, 140, 118, 249,
        190, 114, 71, 9, 151, 44, 72, 54, 15, 171, 30, 139,
    ];

    let mut stack_even: State<SIZE> = black_box(random(seed.clone()));
    let mut stack_odd: State<SIZE> = black_box(new());

    let mut heap_even: Box<State<SIZE>> = black_box(Box::new(random(seed.clone())));
    let mut heap_odd: Box<State<SIZE>> = black_box(Box::new(new()));

    group.bench_function(BenchmarkId::new("stack", SIZE), |b| {
        b.iter(|| {
            tick(&stack_even, &mut stack_odd);
            tick(&stack_odd, &mut stack_even);
        })
    });

    group.bench_function(BenchmarkId::new("heap", SIZE), |b| {
        b.iter(|| {
            tick(&heap_even, &mut heap_odd);
            tick(&heap_odd, &mut heap_even);
        })
    });
}

fn stack_vs_heap(c: &mut Criterion) {
    let mut group = c.benchmark_group("stack_vs_heap");

    compare_ticks::<8>(&mut group);
    compare_ticks::<16>(&mut group);
    compare_ticks::<32>(&mut group);
    compare_ticks::<64>(&mut group);
    compare_ticks::<128>(&mut group);

    group.finish();
}

criterion_group!(bench, stack_vs_heap);
criterion_main!(bench);
