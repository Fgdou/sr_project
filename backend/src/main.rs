use tokio::net::TcpListener;
use tokio_websockets::{Error, ServerBuilder};
use futures_util::{SinkExt, StreamExt};

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Hello, world!");

    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("Websocket server started on ws://0.0.0.0:8080");

    let (conn, _) = listener.accept().await?;
    let mut server = ServerBuilder::new().accept(conn).await?;

    while let Some(Ok(item)) = server.next().await {
        println!("Received: {item:?}");
        server.send(item).await?;
    }

    Ok(())
}
