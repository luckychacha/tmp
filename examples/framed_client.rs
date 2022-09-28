use bytes::Bytes;
use futures::{StreamExt, SinkExt};
use tokio::net::TcpStream;
use tokio_util::codec::{LengthDelimitedCodec, Framed};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let stream  = TcpStream::connect("127.0.0.1:9527").await?;

    let mut stream = Framed::new(stream, LengthDelimitedCodec::new());

    stream.send(Bytes::from("hello world")).await?;

    if let Some(Ok(data)) = stream.next().await {
        println!("Got: {:?}", String::from_utf8_lossy(&data));
    }

    Ok(())
}