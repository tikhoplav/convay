use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};
use conway::{State, new, random, tick};

fn state_update_bench(c: &mut Criterion) {
    // sha256 "Hello, World!!"
    let seed: [u8; 32] = [
          3, 127, 146, 120,  48, 195,  83,  11,
        249, 175,   8,  97,  14,  74, 238, 206,
         54, 140, 118, 249, 190, 114,  71,   9,
        151,  44,  72,  54,  15, 171,  30, 139
    ];

    let mut even: Box<State<1024>> = black_box(Box::new(random(seed)));
    let mut odd: Box<State<1024>> = black_box(Box::new(new()));
    
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
