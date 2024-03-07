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
given time is in one of two possible states: `live` (`1`) or `dead` (`0`).

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
(or a simulation step) would be referred simply as *State*. Each new *State* 
(or a generation) is evaluated by applying the following rules to each *Cell* of
the preceding one:

- A `live` *Cell* with 2 or 3 neighbors stays `live`, otherwise becomes `dead`;
- A `dead` *Cell* with exactly 3 `live` neighbors becomes `live`;

The rules are applied to the all *Cells* simultaniously at a descrete moment (a 
*Tick*). The rules are continue to be applied repeatedly to create further
generations.

<br>

### Requirements

The goal of the project is to implement a Conway's Game of Life web application,
with server and simualtion built in `Rust` and client built with `Webgpu`, that
allows:

- Up to 16 `Websocket` based simultanious connections;
- To observe a simulation of a 1920x1080 *World* at at least 30 FPS rate;
- To start / restart a simulation with an arbitrary *seed*.

<br>
<br>

## Implementation design overview

Past experience of implementing Conway's Game of Life showed that a naive way
(in which each individual *Cell's* state is computed one at a time) in not 
performant enough to fulfill the FPS requirement. 30 FPS means that a client
must receive a new *State* as frequent as once per each 0.5 seconds.

Considering that a client should also process the incoming packed *State* and 
render it onto a canvas (which depends a lot on the client's host machine),
the approximate time for a packet should be about 400ms. 

Considering a network latency (which again may vary a lot from client to client)
as about 300ms, gives 100ms in total to perform a simulation step and to pack
and broadcast the new *State* to all clients by the server app.

Thus being said there are a number of optimizations that should be applied from
the get go:

- Encode each *Cell* of a *State* with a single byte when forming a packet.
  With the *World* size of 1920x1080 that already is ~260Kb per packet. In case
  of bad benchmarks, gzipping may worth considering;

- Utilize as much CPU cache during generation as possible (a value of 64 bytes
  can be used as target buffer size, as a usual size of a cache slot);

- Avoid copying *State* as much as possible (Use two byte arrays for previous
  and current generation, and switch them between ticks without allocating any
  new memory. Considering the first hint, *Cell* should also be stored in mem
  as a single byte, to preven additional "packing" step);

- Utilize multithreading where possible (When broadcasting a *State*, multiple
  sockets may read it in parallel. Probably different regions of the *World*
  may be also processed in parallel during generation);

<br>
<br>

## Roadmap

- [ ] The "Bit State":
  - [ ] Conway's sim step on a 3x3 byte (strip) matrix, producing 1 byte (8 cells) result;
  - [ ] Conway's sim on a chunk of 8x8 strips (64 bytes, 512 cells);
- [ ] Generation procedure:
  - [ ] Chunking of a state with 8x8 strips;
  - [ ] Workload balancing between multiple threads;
- [ ] Simple websocket broadcaster:
  - [ ] State sharing between socket threads;
  - [ ] Signalling of which *State* to broadcast;
- [ ] Simple `Webgpu` renderer:
  - [ ] Render to a texture;
  - [ ] Render texture on a plane;
  - [ ] Camera;
- [ ] Simple websocket receiver;
- [ ] Simple client;

<br>
<br>
<br>
<br>
