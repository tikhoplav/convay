# Conway - Perf sim

The binary to perform `perf` tests agains the library

<br>

## Setup

```
cargo build --release
perf stat -e branches,branch-misses,cache-misses,cycles,instructions,task-clock,L1-dcache-load-misses,L1-icache-load-misses target/release/perfsim
```

<br>

## Optimizations and performance baseline

Results of performance tests right after the optimized (chunked)
engine were introduced:

```
Performance counter stats for 'target/release/perfsim':

       441,480,308      branches                  #   14.917 M/sec                    (71.45%)
           549,651      branch-misses             #    0.12% of all branches          (71.44%)
         2,780,835      cache-misses                                                  (71.42%)
   117,723,754,759      cycles                    #    3.978 GHz                      (71.39%)
   394,102,557,253      instructions              #    3.35  insn per cycle           (71.41%)
         29,594.98 msec task-clock                #    0.999 CPUs utilized          
       413,185,288      L1-dcache-load-misses                                         (71.45%)
           878,649      L1-icache-load-misses                                         (71.44%)

      29.627121954 seconds time elapsed

      29.568635000 seconds user
       0.023997000 seconds sys
```
