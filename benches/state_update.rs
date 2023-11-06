use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};
use conway::{State, new, random, tick};

fn state_update_bench(c: &mut Criterion) {
    let mut even: State<128> = black_box(random());
    let mut odd: State<128> = black_box(new());

    c.bench_function(
        "state_update",
        |b| b.iter(|| {
            tick(&even, &mut odd);
            tick(&odd, &mut even);
        })
    );
}

criterion_group!(bench, state_update_bench);
criterion_main!(bench);
