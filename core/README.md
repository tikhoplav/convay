# Conway's game of life.

![demo](https://github.com/tikhoplav/convay/assets/62797411/a04b2115-a197-44a7-8a0d-61c80be31f82)

This project is a tribute to [Coway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)
authored by John Horton Conway.

<br>
<br>

## What this project is about. 

There are multiple implementations of Conway's Game of Life that exists, like:

- [Hashlife](https://en.wikipedia.org/wiki/Hashlife);
- [QuickLife](https://conwaylife.com/wiki/QuickLife);
- [Larger than Life](https://conwaylife.com/wiki/Larger_than_Life);

Those were designed to efficiently compute generations over generations accross
significant amounts of steps in order to check a world's state at some distant
point in the future. This project, on the other hand, is focused on real-time 
simulation, just like most of the earliest algorithms.

The main idea is not only to keep the simulation running, but also to provide a 
way for multiple agents to observe the simulation at the same time and, probably,
even to allow to interract with the world changing the course of the simulation.

<br>

### Rules

The concept of a *Cell* is taken directly from the Conway's definition, as a  
unit of the two dimenstional orthogonal space (a *World*), which at any given
time is in one of two possible states: `live` (`1`) or `dead` (`0`).

The set of states of all the *Cells* of the *World* at any given moment in time 
(or a simulation step) would be referred simply as a *State*. Each new *State* 
(or a generation) is evaluated by applying the following rules to each *Cell* of
the preceding *State*:

- A `live` *Cell* with 2 or 3 adjacent `live` *Cells* stays `live`, otherwise `dies`;
- A `dead` *Cell* with exactly 3 `live` adjacent *Cells* becomes `live`;

The rules are applied to the all *Cells* simultaneously at a descrete moment (a 
*Tick*). The rules are continue to be applied repeatedly resulting in new 
generations to be produced.

<br>
<br>

## Requirements

This library must provide the following:

- [ ] The ability to create a *State* with the specified dimensions;
- [ ] The ability to fill a *State* with pseudo-randomly generated *Cells*;
- [ ] The ability to compute a \*chunk of the next *State* using some \*\*chunk
      of a preceding *State*;
- [ ] The ability to read a *State* as a contiguous slice of bytes\*\*\*;

\* - The procedure of generating a new *State* should be available for
multi-threaded computation. The best would be if, the whole procedure can be
split evenly accross multiple independent processes.

\*\* - The input chunk and output chunk is not necessary to be the same sizes
(dimensions). Those size are subjects of computational optimizations.

\*\*\* - In cases when the *State* needs to be transfered over the wires, it 
is required to be byte serializable. More over when represented as contiguous
slice / array of bytes, the decoding side must be able to restore the *State*
by knowing just its dimensions.

<br>

The library has to satisfy the following:

- [ ] `no_std`, as it should be available for compiling to `WASM`;
- [ ] The pseudo-random mechanism must be deterministic, providing the same
      *State* with the same seed on all platforms / targets.

<br>
<br>
<br>
<br>
