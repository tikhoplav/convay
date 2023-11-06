# Conway's game of life.

![demo](https://github.com/tikhoplav/convay/assets/62797411/a04b2115-a197-44a7-8a0d-61c80be31f82)

This project is a tribute to [Coway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) authored by John Horton Conway.

## Roadmap

- [x] Implement basic simulation engine;
- [x] Implement basic networking;
- [x] Implement basic rendering engine;
- [x] Improve rendrer to use only cell index (reduce buffer);

<br>
<br>

### Update Oct 2023

- [] Re-implement the state engine;
    - [] Reduce memory allocations;
    - [] Optimize cache usage (chunks maybe?);

- [] Re-implement networking;
    - [] Change serialization implementation to write into a given slice;
    - [] Intriduce thread-safe (only main thread writes) shared byte buffer;
    - [] Channels are used to receive only a `ready` signal, and should read
         from the shared buffer instead;

- [] Re-implement the renderer;
    - [] Remove the state / coordinates computation and instead render a raw
         state (from the socket) onto a monochrome texture, applied to a 
         singular rectangular;
    - [] The camera position and zoom should take effect on the rect position
         and scale instead;

<br>
<br>
<br>
<br>
