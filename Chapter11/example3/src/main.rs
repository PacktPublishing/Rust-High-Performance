extern crate tokio;

use tokio::prelude::*;
use tokio::net::TcpListener;
use tokio::io;

fn main() {
    let address = "127.0.0.1:8000".parse().unwrap();
    let listener = TcpListener::bind(&address).unwrap();

    let server = listener
        .incoming()
        .map_err(|e| eprintln!("Error accepting connection: {:?}", e))
        .for_each(|socket| {
            let (reader, writer) = socket.split();
            let copied = io::copy(reader, writer);

            let handler = copied
                .map(|(count, _reader, _writer)| println!("{} bytes received", count))
                .map_err(|e| eprintln!("Error: {:?}", e));

            tokio::spawn(handler)
        });

    tokio::run(server);
}
