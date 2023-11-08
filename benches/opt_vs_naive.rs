use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
};
use conway::{State, new_state, random_state, tick};
use conway::naive_state;

fn opt_vs_naive(c: &mut Criterion) {
    let mut naive_even: Box<naive_state::State<1024>> = black_box(
        Box::new(naive_state::random(b"Hello, World!!"))
    );
    let mut naive_odd: Box<naive_state::State<1024>> = black_box(
        Box::new(naive_state::new())
    );

    let mut even: Box<State<128, 1024>> = black_box(
        Box::new(random_state(b"Hello, World!!"))
    );
    let mut odd: Box<State<128, 1024>> = black_box(Box::new(new_state()));


    let mut group = c.benchmark_group("opt_vs_naive");

    group.bench_function("naive", |b| {
        b.iter(|| {
            naive_state::tick(&naive_even, &mut naive_odd);
            naive_state::tick(&naive_odd, &mut naive_even);
        })
    });

    group.bench_function("opt", |b| {
        b.iter(|| {
            tick(&even, &mut odd);
            tick(&odd, &mut even);
        })
    });


    group.finish();
}

criterion_group!(bench, opt_vs_naive);
criterion_main!(bench);
