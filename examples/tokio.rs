extern crate tokio_crate as tokio;

use bytes::BytesMut;
use futures::stream::StreamExt;
use serial_io::{build, AsyncSerial, TTYPort};
use std::{env, io, str};
use tokio_util::codec::{Decoder, Encoder};

#[cfg(unix)]
const DEFAULT_TTY: &str = "/dev/ttyUSB0";
#[cfg(windows)]
const DEFAULT_TTY: &str = "COM1";

struct LineCodec;

impl Decoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let newline = src.as_ref().iter().position(|b| *b == b'\n');
        if let Some(n) = newline {
            let line = src.split_to(n + 1);
            return match str::from_utf8(line.as_ref()) {
                Ok(s) => Ok(Some(s.to_string())),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Invalid String")),
            };
        }
        Ok(None)
    }
}

impl Encoder<String> for LineCodec {
    type Error = io::Error;

    fn encode(&mut self, _item: String, _dst: &mut BytesMut) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[tokio::main(flavor = "current_thread")]
pub async fn main() {
    let mut args = env::args();
    let tty_path = args.nth(1).unwrap_or_else(|| DEFAULT_TTY.into());

    let builder = build(tty_path, 9600);
    let port = TTYPort::open(&builder).unwrap();
    let mut rx = AsyncSerial::from_serial(port).unwrap();

    #[cfg(unix)]
    rx.set_exclusive(false)
        .expect("Unable to set serial port exclusive to false");

    let mut reader = LineCodec.framed(rx);

    while let Some(line_result) = reader.next().await {
        let line = line_result.expect("Failed to read line");
        println!("{}", line);
    }
}
