use anyhow::{anyhow, Result};
use std::{sync::mpsc, thread, time::Duration};

const NUM_PRODUCERS: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Message {
    index: usize,
    value: usize,
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    // 创建producers
    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || {
            let _ = producer(tx, i);
        });
    }
    drop(tx);

    // 创建consumer
    let consumer = thread::spawn(|| {
        // 如果tx没有释放, 这个for循环不会退出而是一直等待
        for msg in rx {
            println!("consume {:?}", msg);
        }
        42
    });

    let last_words = consumer
        .join()
        .map_err(|e| anyhow!("Thread join error: {:?}", e))?;
    println!("Consumer exit with {}", last_words);
    Ok(())
}

fn producer(tx: mpsc::Sender<Message>, index: usize) -> Result<()> {
    loop {
        let value = rand::random::<usize>();
        tx.send(Message::new(index, value))?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(sleep_time));
        if value % 5 == 0 {
            break;
        }
    }
    println!("producer {} exit", index);
    Ok(())
}

impl Message {
    fn new(index: usize, value: usize) -> Self {
        Message { index, value }
    }
}
