use std::{thread, time::Duration};

use anyhow::{Ok, Result};
use concurrency::Metrics;
use rand::Rng;

const N: usize = 10;
const M: usize = 10;

fn main() -> Result<()> {
    let m = Metrics::new();
    println!("{:?}", m.snapshot());

    for idx in 0..N {
        task_worker(idx, m.clone())?;
    }

    for _ in 0..M {
        request_worker(m.clone())?;
    }

    loop {
        thread::sleep(Duration::from_secs(5));
        println!("{:?}\n", m.snapshot());
    }
}

fn task_worker(idx: usize, m: Metrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();

            thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
            m.incr(format!("call.thread.worker.{}", idx))?;
        }
        #[allow(unreachable_code)]
        Ok(())
    });
    Ok(())
}

fn request_worker(m: Metrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();

            thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
            let page = rng.gen_range(0..255);
            m.incr(format!("req.page.{}", page))?;
        }
        #[allow(unreachable_code)]
        Ok(())
    });
    Ok(())
}