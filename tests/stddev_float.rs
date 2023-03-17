use std::time::{Instant, Duration};

use fast_stats::stats::stddev;
use fast_stats::fstats_float::Stats;

#[test]
fn stddev_stats_benchmark() {
    let vs: Vec<i32> = (0..200000).collect();
    let mut vs: Vec<f64> = vs.iter().map(|x| *x as f64).collect();
    let es: Vec<i32> = (200000..200010).collect();
    let es: Vec<f64> = es.iter().map(|x| *x as f64).collect();
    vs.extend(es);

    let mut i = 10;
    let mut elapsed_vec = vec![];
    while i > 0 {
        let now = Instant::now();
        stddev(&vs);
        let elapsed = now.elapsed();
        elapsed_vec.push(elapsed);
        i -= 1;
    }
    let total_elapsed: Duration = elapsed_vec.iter().sum();
    let avg_elapsed = total_elapsed.as_nanos() / 10;
    println!("Averaged elapsed time is {}", avg_elapsed) // ~11059856
}

#[test]
fn stddev_fstats_benchmark() {
    let mut vfs = Stats::new();
    let v: Vec<i32> = (0..200000).collect();
    let v: Vec<f64> = v.iter().map(|x| *x as f64).collect();
    vfs.push_vec(v);
    vfs.stddev();
    
    let v: Vec<i32> = (200000..200010).collect();
    let v: Vec<f64> = v.iter().map(|x| *x as f64).collect();
    vfs.push_vec(v);

    let mut i = 10;
    let mut elapsed_vec = vec![];
    while i > 0 {
        let now = Instant::now();
        vfs.stddev();
        let elapsed = now.elapsed();
        elapsed_vec.push(elapsed);
        i -= 1;
    }
    let total_elapsed: Duration = elapsed_vec.iter().sum();
    let avg_elapsed = total_elapsed.as_nanos() / 10;
    println!("Averaged elapsed time is {}", avg_elapsed) // ~49
    
}

#[test]
fn drain_stddev_stats_benchmark() {
    let vs: Vec<i32> = (0..2000010).collect();
    let mut vs: Vec<f64> = vs.iter().map(|x| *x as f64).collect();
    vs.drain(0..10);

    let mut i = 10;
    let mut elapsed_vec = vec![];
    while i > 0 {
        let now = Instant::now();
        stddev(&vs);
        let elapsed = now.elapsed();
        elapsed_vec.push(elapsed);
        i -= 1;
    }
    let total_elapsed: Duration = elapsed_vec.iter().sum();
    let avg_elapsed = total_elapsed.as_nanos() / 10;
    println!("Averaged elapsed time is {}", avg_elapsed) // ~98569335
}

#[test]
fn drain_stddev_fstats_benchmark() {
    let mut vfs = Stats::new();
    let v: Vec<i32> = (0..200010).collect();
    let v: Vec<f64> = v.iter().map(|x| *x as f64).collect();
    vfs.push_vec(v);
    vfs.stddev();
    vfs.drain(0..10);

    let mut i = 10;
    let mut elapsed_vec = vec![];
    while i > 0 {
        let now = Instant::now();
        vfs.stddev();
        let elapsed = now.elapsed();
        elapsed_vec.push(elapsed);
        i -= 1;
    }
    let total_elapsed: Duration = elapsed_vec.iter().sum();
    let avg_elapsed = total_elapsed.as_nanos() / 10;
    println!("Averaged elapsed time is {}", avg_elapsed) // ~49
    
}