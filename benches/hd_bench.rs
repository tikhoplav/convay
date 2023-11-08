use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};
use conway::{State, new_state, random_state, tick};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut even: Box<State<240, 1080>> = black_box(
        Box::new(random_state(b"HD we love!!"))
    );

    let mut odd: Box<State<240, 1080>> = black_box(
        Box::new(new_state())
    );

    let mut i = 0;
    c.bench_function("1920x1080", |b| b.iter(|| {

        // The full cycle is two ticks, but the goal of this benchmark
        // is to reveal a FPS equivalent, that's why it's halved.
        match i % 2 {
            0 => { tick(&even, &mut odd); },
            _ => { tick(&odd, &mut even); }
        };

        i += 1;
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
