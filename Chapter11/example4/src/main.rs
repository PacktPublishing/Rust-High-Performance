extern crate bytes;
extern crate tokio;
extern crate tokio_io;

use std::io;

use tokio_io::codec::{Decoder, Encoder};
use bytes::BytesMut;
use tokio::prelude::*;
use tokio::net::TcpListener;

fn main() {
    let address = "127.0.0.1:8000".parse().unwrap();
    let listener = TcpListener::bind(&address).unwrap();

    let server = listener
        .incoming()
        .map_err(|e| eprintln!("Error accepting connection: {:?}", e))
        .for_each(|socket| {
            tokio::spawn(
                socket
                    .framed(ADividerCodec::default())
                    .for_each(|chunk| {
                        println!("{}", chunk);
                        Ok(())
                    })
                    .map_err(|e| eprintln!("Error: {:?}", e)),
            )
        });

    println!("Running Tokio server...");
    tokio::run(server);
}

#[derive(Debug, Default)]
struct ADividerCodec {
    next_index: usize,
}

impl Decoder for ADividerCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if let Some(new_offset) = buf[self.next_index..].iter().position(|b| *b == b'a') {
            let new_index = new_offset + self.next_index;
            let res = buf.split_to(new_index + 1);
            let res = &res[..res.len() - 1];
            let res: Vec<_> = res.into_iter()
                .cloned()
                .filter(|b| *b != b'\r' && *b != b'\n')
                .collect();
            let res = String::from_utf8(res).map_err(|_| {
                io::Error::new(io::ErrorKind::InvalidData, "Unable to decode input as UTF8")
            })?;
            self.next_index = 0;
            Ok(Some(res))
        } else {
            self.next_index = buf.len();
            Ok(None)
        }
    }

    fn decode_eof(&mut self, buf: &mut BytesMut) -> Result<Option<String>, io::Error> {
        Ok(match self.decode(buf)? {
            Some(frame) => Some(frame),
            None => {
                // No terminating 'a' - return remaining data, if any
                if buf.is_empty() {
                    None
                } else {
                    let res = buf.take();
                    let res: Vec<_> = res.into_iter()
                        .filter(|b| *b != b'\r' && *b != b'\n')
                        .collect();
                    let res = String::from_utf8(res).map_err(|_| {
                        io::Error::new(io::ErrorKind::InvalidData, "Unable to decode input as UTF8")
                    })?;
                    self.next_index = 0;
                    Some(res)
                }
            }
        })
    }
}

impl Encoder for ADividerCodec {
    type Item = String;
    type Error = io::Error;

    fn encode(&mut self, chunk: Self::Item, buf: &mut BytesMut) -> Result<(), io::Error> {
        use bytes::BufMut;

        buf.reserve(chunk.len() + 1);
        buf.put(chunk);
        buf.put_u8(b'a');
        Ok(())
    }
}
