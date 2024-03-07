# Conway's game of life.

![demo](https://github.com/tikhoplav/convay/assets/62797411/a04b2115-a197-44a7-8a0d-61c80be31f82)

This project is a tribute to [Coway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)
authored by John Horton Conway.

<br>
<br>

## What this project is about. 

There are multiple multiple implementations of the Conway's Game of Life out
there, some of which are highly optimized for the simulation, like
[Hashlife](https://en.wikipedia.org/wiki/Hashlife), however, this project's 
goal is to build a multiplayer version of the Conway's Game of Life, hence it's 
implementation would stick to the original ideas of simulation (cell centric).

"What is the multiplayer Conway's Game of Life?" you might ask. The main idea 
is not to only keep a simulation running, but to also broadcast the simulation
to a number of observers (player) at the real time. Later this may be extended
to a degree where players would be able to even interact with the world,
placing cells and cellular structures, so that simulation would catch them up.

<br>

### Rules

The concept of a *Cell* is taken directly from the Conway's definition, it is a
unit of the two dimenstional orthogonal the space (a *World*), which at any
given time is in one of two possible states: `live` (`1`) or `dead` (0).

Every *Cell* interacts with 8 adjacent *Cells*, it's neighbors, as showed in 
the scheme bellow. A `dead` *Cell* can also be reffered as an `empty`, as the
*Cell* surrounded only by `dead` *Cells* can be reffered as "with no neighbors"

```
.......
.......
..###..         X - A Cell
..#X#..         # - The Cell's neighbor
..###..         . - non-neighbors
.......
.......
```

<br>

The set of states of all the *Cells* of the *World* at any given moment in time 
(or a simulation step) would be referred simply as *State*.

Each new *State* (or generation) is evaluated by applying the following rules
to each *Cell* of the preceding one:

- A `live` *Cell* with 2 or 3 neighbors stays `live`, otherwise dies;
- A `dead` *Cell* with exactly 3 `live` neighbors becomes `live`;

The rules are applied to the all *Cells* simultaniously at a descrete moment (a 
*Tick*). The rules are continue to be applied repeatedly to create further
generations.

<br>
<br>

## Roadmap

- [ ] Implement the "Bit State":
    - [ ] Conway's sim step on a 3x3 byte matrix, producing 1 byte result; 

<br>
<br>
<br>
<br>
