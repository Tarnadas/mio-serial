//! Tests for the Unix impl of `mio_serial::unix::Serial`
#![cfg(unix)]

extern crate serial_io;
extern crate serialport;

use std::io::{Read, Write};
use std::os::unix::prelude::*;
use std::str;

use serial_io::{build, Serial};

fn get_available_serialport_name() -> Option<String> {
    match serial_io::available_ports() {
        Err(_) => None,
        Ok(ports) => ports.into_iter().map(|s| s.port_name).next(),
    }
}

#[test]
#[ignore]
fn test_from_builder() {
    let tty_path = get_available_serialport_name().expect("No available serial ports.");

    let builder = build(tty_path, 9600);

    let serial = Serial::from_builder(&builder).expect("Unable to wrap TTYPort.");

    assert!(serial.as_raw_fd() > 0, "Illegal file descriptor.");
}

#[test]
fn test_serial_pair() {
    let (mut master, mut slave) = Serial::pair().expect("Unable to create ptty pair");

    // Test file descriptors.
    assert!(
        master.as_raw_fd() > 0,
        "Invalid file descriptor on master ptty"
    );
    assert!(
        slave.as_raw_fd() > 0,
        "Invalid file descriptor on slave ptty"
    );
    assert_ne!(
        master.as_raw_fd(),
        slave.as_raw_fd(),
        "master and slave ptty's share the same file descriptor."
    );

    let msg = "Test Message";
    let mut buf = [0u8; 128];

    // Write the string on the master
    assert_eq!(
        master.write(msg.as_bytes()).unwrap(),
        msg.len(),
        "Unable to write message on master."
    );

    // Read it on the slave
    let nbytes = slave.read(&mut buf).expect("Unable to read bytes.");
    assert_eq!(
        nbytes,
        msg.len(),
        "Read message length differs from sent message."
    );

    assert_eq!(
        str::from_utf8(&buf[..nbytes]).unwrap(),
        msg,
        "Received message does not match sent"
    );
}
