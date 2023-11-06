use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
    BenchmarkId,
};
use conway::{State, new, random, tick};

fn stack_vs_heap(c: &mut Criterion) {
    // sha256 "Hello, World!!"
    let seed: [u8; 32] = [
          3, 127, 146, 120,  48, 195,  83,  11,
        249, 175,   8,  97,  14,  74, 238, 206,
         54, 140, 118, 249, 190, 114,  71,   9,
        151,  44,  72,  54,  15, 171,  30, 139
    ];
    let seed2 = seed.clone();

    let mut stack_even: State<128> = black_box(random(seed));
    let mut stack_odd: State<128> = black_box(new());

    let mut heap_even: Box<State<128>> = black_box(
        Box::new(random(seed2))
    );
    let mut heap_odd: Box::<State<128>> = black_box(
        Box::new(new())
    );


    let mut group = c.benchmark_group("stack_vs_heap");

    // TODO:: Consider adding comparison based on the size

    group.bench_function(
        BenchmarkId::new("stack", 128), 
        |b| b.iter(|| {
            tick(&stack_even, &mut stack_odd);
            tick(&stack_odd, &mut stack_even);
        })
    );

    group.bench_function(
        BenchmarkId::new("heap", 128),
        |b| b.iter(|| {
            tick(&heap_even, &mut heap_odd);
            tick(&heap_odd, &mut heap_even);
        })
    );

    group.finish();
}

criterion_group!(bench, stack_vs_heap);
criterion_main!(bench);
