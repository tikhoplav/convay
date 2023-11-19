# Conway - Perf sim

The binary to perform `perf` tests agains the library

<br>

## Setup

```
cargo build --release
perf stat target/release/perfsim
```

<br>

## Optimizations and performance baseline

Results of performance tests right after the optimized (chunked)
engine were introduced:

```
Starting a 1024x1024 simulation for 100000 steps
Succesfully simulated 100000 steps

 Performance counter stats for 'target/release/perfsim':

         29,478.11 msec task-clock                #    1.000 CPUs utilized          
               279      context-switches          #    9.465 /sec                   
                 0      cpu-migrations            #    0.000 /sec                   
               200      page-faults               #    6.785 /sec                   
   117,116,047,019      cycles                    #    3.973 GHz                      (83.33%)
       260,246,411      stalled-cycles-frontend   #    0.22% frontend cycles idle     (83.34%)
    81,874,878,177      stalled-cycles-backend    #   69.91% backend cycles idle      (83.34%)
   394,155,639,577      instructions              #    3.37  insn per cycle         
                                                  #    0.21  stalled cycles per insn  (83.34%)
       435,354,263      branches                  #   14.769 M/sec                    (83.34%)
           371,021      branch-misses             #    0.09% of all branches          (83.32%)

      29.481720845 seconds time elapsed

      29.478538000 seconds user
       0.000000000 seconds sys
```
