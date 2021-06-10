//! Simple example that echoes received serial traffic to stdout

#[cfg(unix)]
use mio::{Events, Interest, Poll, Token};
use serial_io::{build, Serial, TTYPort};
use std::env;
use std::io;
use std::io::Read;
use std::str;

const SERIAL_TOKEN: Token = Token(0);

#[cfg(unix)]
const DEFAULT_TTY: &str = "/dev/ttyUSB0";
#[cfg(windows)]
const DEFAULT_TTY: &str = "COM1";

#[cfg(windows)]
fn is_closed(state: Ready) -> bool {
    false
}

pub fn main() {
    let mut args = env::args();
    let tty_path = args.nth(1).unwrap_or_else(|| DEFAULT_TTY.into());

    let mut poll = Poll::new().unwrap();
    let mut events = Events::with_capacity(1024);

    // Create the listener
    println!("Opening {} at 9600,8N1", &tty_path);
    let builder = build(tty_path, 9600);
    let port = TTYPort::open(&builder).unwrap();
    let mut rx = Serial::from_serial(port).unwrap();

    poll.registry()
        .register(&mut rx, SERIAL_TOKEN, Interest::READABLE)
        .unwrap();

    let mut rx_buf = [0u8; 1024];

    'outer: loop {
        if let Err(ref e) = poll.poll(&mut events, None) {
            println!("poll failed: {}", e);
            break;
        }

        if events.is_empty() {
            println!("Read timed out!");
            continue;
        }

        for event in events.iter() {
            match event.token() {
                SERIAL_TOKEN => {
                    if event.is_read_closed() {
                        println!("Quitting due to event: {:?}", event);
                        break 'outer;
                    }
                    if event.is_readable() {
                        // With edge triggered events, we must perform reading until we receive a WouldBlock.
                        // See https://docs.rs/mio/0.6/mio/struct.Poll.html for details.
                        loop {
                            match rx.read(&mut rx_buf) {
                                Ok(count) => {
                                    println!("{:?}", String::from_utf8_lossy(&rx_buf[..count]))
                                }
                                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                    break;
                                }
                                Err(ref e) => {
                                    println!("Quitting due to read error: {}", e);
                                    break 'outer;
                                }
                            }
                        }
                    }
                }
                t => unreachable!("Unexpected token: {:?}", t),
            }
        }
    }
}
