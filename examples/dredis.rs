use anyhow::Result;
use core::net::SocketAddr;
use tokio::{
    self,
    io::{self, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
use tracing::{info, warn};

const BUF_SIZE: usize = 4096;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = "127.0.0.1:6379";

    let listener = TcpListener::bind(addr).await?;
    info!("Dredis Listening on {}", addr);

    loop {
        let (stream, raddr) = listener.accept().await?;
        info!("New connection from {}", addr);
        tokio::spawn(async move {
            if let Err(e) = process_redis_conn(stream, raddr).await {
                warn!("Error processing connection with {} err: {}", addr, e);
            }
        });
    }

    #[allow(unreachable_code)]
    Ok(())
}

async fn process_redis_conn(mut stream: TcpStream, raddr: SocketAddr) -> Result<()> {
    loop {
        // Wait for the socket to be readable
        stream.readable().await?;

        let mut buf = Vec::with_capacity(BUF_SIZE);

        // Try to read data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match stream.try_read_buf(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                println!("read {} bytes", n);
                let line = String::from_utf8_lossy(&buf);
                info!("read line {}", line);
                stream.write_all(b"+Ok\r\n").await?;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
    warn!("Connection closed {}", raddr);
    Ok(())
}
