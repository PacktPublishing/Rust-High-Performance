extern crate futures;
extern crate tokio_core;
extern crate websocket;

use websocket::message::OwnedMessage;
use websocket::server::InvalidConnection;
use websocket::async::Server;

use tokio_core::reactor::Core;
use futures::{Future, Sink, Stream};

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let server = Server::bind("127.0.0.1:2794", &handle).unwrap();

    let task = server
        .incoming()
        .map_err(|InvalidConnection { error, .. }| error)
        .for_each(|(upgrade, addr)| {
            println!("Got a connection from: {}", addr);

            if !upgrade.protocols().iter().any(|s| s == "rust-websocket") {
                handle.spawn(
                    upgrade
                        .reject()
                        .map_err(|e| println!("Error: '{:?}'", e))
                        .map(|_| {}),
                );
                return Ok(());
            }

            let fut = upgrade
                .use_protocol("rust-websocket")
                .accept()
                .and_then(|(client, _)| {
                    let (sink, stream) = client.split();

                    stream
                        .take_while(|m| Ok(!m.is_close()))
                        .filter_map(|m| match m {
                            OwnedMessage::Ping(p) => Some(OwnedMessage::Pong(p)),
                            OwnedMessage::Pong(_) => None,
                            _ => Some(m),
                        })
                        .forward(sink)
                        .and_then(|(_, sink)| sink.send(OwnedMessage::Close(None)))
                });

            handle.spawn(fut.map_err(|e| println!("Error: {:?}", e)).map(|_| {}));
            Ok(())
        });

    core.run(task).unwrap();
}
