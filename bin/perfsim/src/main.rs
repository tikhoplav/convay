//! Conway - perf sim
//!
//! This crate is used to run perfromance tests against the libarary
//! code. It contains just a minimal setup to continiously run the 
//! simulation.

use conway::{State, new_state, random_state, tick};

fn main() {

    println!(
        "Starting a {}x{} simulation for {} steps",
        1024,
        1024,
        100_000
    );

    // We use 1024x1024 cell world to run simulations.
    // TODO: make world size to be configurable throug
    // input arguments or env variables.
    //
    // States are boxed to match how it is usually set
    // in the actual binary.
    let mut even: Box<State<128, 1024>> = Box::new(
        random_state(b"Hello, perf!!")
    );
    let mut odd: Box<State<128, 1024>> = Box::new(new_state());

    let mut step: usize = 0;
    while step < 100_000 {
        match step % 2 {
            0 => tick(&even, &mut odd),
            _ => tick(&odd, &mut even),
        };

        step += 1;
    }

    println!("Succesfully simulated {} steps", 100_000);
}
