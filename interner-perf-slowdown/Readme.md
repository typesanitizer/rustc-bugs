I noticed a regression in performance of generated code from Rust 1.53.0 to
Rust 1.55.0, and this regression is still present on nightly.
I suspect it's probably down to hash table performance, since there
the benchmark is hash table heavy, but I haven't narrowed it down.

Install toolchains:

```
rustup toolchain install 1.53.0
rustup toolchain install 1.55.0
rustup toolchain install nightly # tested on Sep 25, 73422130e
```

Set up data:

```
git clone --quiet --depth=1 https://github.com/rust-analyzer/rust-analyzer.git --branch=2021-09-20 benchmarks/data/rust-analyzer
rm -rf benchmarks/data/rust-analyzer/.git
```

Run benchmarks with

```
cargo +1.53.0 bench
cargo +1.55.0 bench
cargo +nightly bench
```

Here are the results on an M1 MacBook Pro (2020) running macOS 11.4.

```
// 1.53.0
intern serial: (warm, n = 1, ahash)
  time:   [40.677 ms 40.708 ms 40.747 ms]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

intern serial: (warm, n = 1, fxhash)
  time:   [41.778 ms 41.812 ms 41.847 ms]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

// 1.55.0
intern serial: (warm, n = 1, ahash)
  time:   [43.743 ms 43.794 ms 43.850 ms]
  change: [+7.4233% +7.5789% +7.7384%] (p = 0.00 < 0.05)
  Performance has regressed.
Found 14 outliers among 100 measurements (14.00%)
  1 (1.00%) high mild
  13 (13.00%) high severe

intern serial: (warm, n = 1, fxhash)
  time:   [44.625 ms 44.683 ms 44.747 ms]
  change: [+6.7014% +6.8654% +7.0466%] (p = 0.00 < 0.05)
  Performance has regressed.
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe

// Nightly (Sep 25 @ 73422130e)
intern serial: (warm, n = 1, ahash)
  time:   [43.909 ms 43.956 ms 44.013 ms]
  change: [+0.2008% +0.3719% +0.5363%] (p = 0.00 < 0.05)
  Change within noise threshold.
Found 10 outliers among 100 measurements (10.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  7 (7.00%) high severe

intern serial: (warm, n = 1, fxhash)
  time:   [44.273 ms 44.446 ms 44.716 ms]
  change: [-0.9554% -0.5305% +0.0545%] (p = 0.03 < 0.05)
  Change within noise threshold.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
```
