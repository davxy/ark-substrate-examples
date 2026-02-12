# Ring VRF

## Ring Accumulate and Commit

Ring commitment with buffered keys.

`x` keys are stored in the `RingKeys` buffer before running the benchmark.

We're benchmarking:
1. Accumulation of the `x` buffered keys
2. Final ring commitment

### Pure WASM

Data points distribution:
    x   mean µs  sigma µs       %
    1      3999     34.45    0.8%
   13     20630       512    2.4%
   25     34090     56.37    0.1%
   37     52200     138.8    0.2%
   50     59010     72.72    0.1%

Time ~=     4803
    + x     1158
              µs

### Hostcalls

Data points distribution:
    x   mean µs  sigma µs       %
    1      1336     20.28    1.5%
   13      5729     36.32    0.6%
   25     10380     305.3    2.9%
   37     14630     119.3    0.8%
   50     19090     138.1    0.7%

Time ~=     1062
    + x      364
              µs

## Ring Accumulation

`x` keys are accumulated (no commit)


### Pure WASM

Data points distribution:
    x   mean µs  sigma µs       %
    1      4008     4.756    0.1%
   13     20150     16.23    0.0%
   25     33530       619    1.8%
   37     47460     35.96    0.0%
   50     59020     149.8    0.2%

Time ~=     4500
    + x     1124
              µs

### Hostcalls

Data points distribution:
    x   mean µs  sigma µs       %
    1      1381     40.04    2.8%
   13      5803      36.9    0.6%
   25      9992      5.54    0.0%
   37     14070      73.6    0.5%
   50     18550     102.2    0.5%

Time ~=     1159
    + x    349.3
              µs

## Ring Commit

### Pure WASM

Time ~=    25.22
              µs
 
### Hostcalls

Time ~=    40.54
              µs

## Verification

### Pure WASM

Time ~=    30130
    + x    38.19
              µs

### Hostcalls

Time ~=    17190
    + x    14.58
              µs

## Verification (Naive Batching)

### Pure WASM

Data points distribution:
    x   mean µs  sigma µs       %
    1     30140     88.23    0.2%
    2     55910      2540    4.5%
    3     76610      2964    3.8%
    4    100900      4921    4.8%
    5    128900      6964    5.3%

Time ~=     5710
    + x    24260
              µs

### Hostcalls

Data points distribution:
    x   mean µs  sigma µs       %
    1     19160      1455    7.5%
    2     29810      2725    9.1%
    3     43660      3998    9.1%
    4     52990      1914    3.6%
    5     65920      2690    4.0%

Time ~=     7299
    + x    11670
              µs

---

# IETF VRF

## Verification

### Pure WASM

Time ~=     2182
              µs

### Hostcalls

Time ~=    666.8
              µs
