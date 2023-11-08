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

- [x] Re-implement the state engine;
    - [x] Reduce memory allocations;
    - [x] Optimize cache usage (chunks maybe?);

![image](https://github.com/tikhoplav/conway/assets/62797411/afd7d8c6-7810-4df7-ac1b-3eccf7dfd6ad)

Benchmark is performed on a state 1024x1024 cells, the fact that optimized
state takes 8 times less space is not accounted:

```
$ cargo bench --bench opt_vs_naive
    Finished bench [optimized] target(s) in 0.02s
     Running benches/opt_vs_naive.rs (target/release/deps/opt_vs_naive-a5d6f31f8a938906)
opt_vs_naive/naive      time:   [3.4400 ms 3.4480 ms 3.4563 ms]
                        change: [+0.6129% +0.8793% +1.1250%] (p = 0.00 < 0.05)
                        Change within noise threshold.
opt_vs_naive/opt        time:   [683.84 µs 684.10 µs 684.38 µs]
                        change: [-0.3257% -0.1698% -0.0422%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 7 outliers among 100 measurements (7.00%)
  7 (7.00%) high mild
```

<br>

- [ ] Re-implement networking;
    - [ ] Change serialization implementation to write into a given slice;
    - [ ] Intriduce thread-safe (only main thread writes) shared byte buffer;
    - [ ] Channels are used to receive only a `ready` signal, and should read
         from the shared buffer instead;

<br>

- [ ] Re-implement the renderer;
    - [ ] Remove the state / coordinates computation and instead render a raw
         state (from the socket) onto a monochrome texture, applied to a 
         singular rectangular;
    - [ ] The camera position and zoom should take effect on the rect position
         and scale instead;

<br>
<br>
<br>
<br>
