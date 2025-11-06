use std::{hint::black_box, sync::mpsc};

use anyhow::Context;
use rayon::{ThreadPoolBuilder, current_thread_index, prelude::*};

fn fib(n: usize) -> usize {
    if n <= 1 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}

fn heavy_load_black_box(n: usize) -> usize {
    black_box(fib(n))
}

#[derive(Clone, Debug)]
struct ResultData {
    seq_no: usize,
    payload: usize,
    thread_idx: Option<usize>,
}

fn main() -> anyhow::Result<()> {
    let num_cpus = num_cpus::get();
    println!("number of cpus: {num_cpus}");

    ThreadPoolBuilder::new()
        // Default configuration does not guarantee to spawn a thread for each cpu
        .num_threads(num_cpus)
        .build_global()
        .context("build thread pool")?;

    let data = (2..=21).rev().collect::<Vec<_>>();
    println!("input data: {data:?}");

    let (tx, rx) = mpsc::sync_channel::<ResultData>(20);

    // `.collect()` could be used but this way we would loose the order of job completion
    // since it joins all threads first and only then collects the results
    data.into_par_iter()
        .enumerate()
        .for_each(move |(seq_no, _n)| {
            let payload = heavy_load_black_box(seq_no);
            let thread_idx = current_thread_index();

            let res = ResultData {
                payload,
                seq_no,
                thread_idx,
            };

            tx.send(res).expect("channel closed");
        });

    for (res_seq_no, result) in rx.iter().enumerate() {
        println!(
            "seq no: {}, result {}, result seq no: {}, thread idx: {:?}",
            result.seq_no, result.payload, res_seq_no, result.thread_idx
        );
    }

    Ok(())
}
