use std::{thread, time::Duration};

use anyhow::{Ok, Result};
use concurrency::AmapMetrics;
use rand::Rng;

const N: usize = 10;
const M: usize = 10;
const METRIC_NAMES: [&str; 3] = ["call.thread.worker", "req.page", "req"];

fn main() -> Result<()> {
    let m = AmapMetrics::new(&METRIC_NAMES);
    println!("{}", m);

    for idx in 0..N {
        task_worker(idx, m.clone())?;
    }

    for _ in 0..M {
        request_worker(m.clone())?;
    }

    loop {
        thread::sleep(Duration::from_secs(2));
        println!("{}\n", m);
    }
}

fn task_worker(idx: usize, m: AmapMetrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();

            thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
            m.incr(format!("call.thread.worker.{0:>03}", idx))?;
        }
        #[allow(unreachable_code)]
        Ok(())
    });
    Ok(())
}

fn request_worker(m: AmapMetrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();

            thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
            let page = rng.gen_range(0..255);
            m.incr(format!("req.page.{0:>03}", page))?;
        }
        #[allow(unreachable_code)]
        Ok(())
    });
    Ok(())
}
