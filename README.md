Fast Statistics
===============

A Rust library for doing statistical analysis on streaming data.

Introduction
------------

Taking inspiration from the respective nodejs library https://github.com/bluesmoon/node-faststats, fast-stats is a Rust library allowing the calculation of various statistical quantities with an emphasis on speed.  This library also expands on the above by allowing various transformations of the streaming data.

In undertaking statistical analysis on streaming data, fast-stats attempts to leverage two important factors to improve execution time:
1. Statistics should be updated incrementally as data is added to a set as opposed to calculated on the complete set each time a new piece of data is added.  For large datasets (and/or high freqency processing), looping through the entire data of interest may be a bottleneck. 
2. Many statistical quantities are calculated from the same simple underlying building blocks. 

This is achieved by maintaining a running cache of several variables as data is streamed into the structure.  

As an example, the extraordinary increase in speed in calculating the standard deviation is a result of the observation that the variance can be calculated using a simple formula based on an incremental sum of values and an incremental sum of squares of those values.

This is particularly useful in applications that require the calculation of statistical quantities with respect to a contracting, expanding or fixed window of data from a continuously updating stream.

The only trade-off is a small amount of additional memory usage to store these running values.

Usage
-----

The `Stats` struct functions like a vector of data which maintains a running update of various values used in the calculation of statistical quantities. Data is added and deleted from this object using `push`, `pop` and other vector methods listed in the Functionality section below.

```rust
use fast_stats::fstats_f64;

let v = Stats::new();
v.push_vec(vec![4.0, -1.0, 3.0]);
println!("{}", v.mean());
// 2.0
```

Options
-------

In addition to `fstats_f64`, we have included `fstats_float` which is generic in the float type at the expense of a tiny amount of speed.

Functionality
-------------

### Statistics

The following statistical methods are supported:

*  `mean`
*  `stddev`
*  `min`
*  `max`
*  `..`

### Data Transformation

The following methods are currently supported and offer the same functionality (with the exception of drain) to their corresponding method in the standard library: 

*  `append` 
*  `drain`
*  `insert` 
*  `is_empty`
*  `len`
*  `pop`
*  `push`
*  `push_vec`
*  `remove` 
*  `resize`
*  `splice`
*  `split_off`
*  `swap_remove`
*  `truncate`
*  `..`

New methods not in the standard library include:

*  `trim`

which shortens a vector by removing all elements up to a given index.

### Clearing all data

The `reset` method clears out all data.

### Getting the raw data

The `data` method returns a vector of the underlying data.


