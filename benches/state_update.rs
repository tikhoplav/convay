use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};
use convay::State;

fn state_update_bench(c: &mut Criterion) {
    let mut state = black_box(
        State::new(1024)
    );

    c.bench_function(
        "state_update",
        |b| b.iter(|| state.tick())
    );
}

criterion_group!(bench, state_update_bench);
criterion_main!(bench);
