//! Expose a serial port to the machines local network.
//! 
//! Only connects a single TCP client at a time.
//!
//! This is to allow simple interfacing to embedded devices. My specific
//! use-case is to easily interface with an embedded device that runs a http
//! protocol over its serial line. This pass through effectivly turns it into
//! a web server. As for why I am overlaying HTTP over serial? Becuase I need
//! a comprehensive API to the embedded system, it is very easy to implement
//! and the embedded system I am using has the room to easily handle this.
//! 
//! Use Ctrl-C to exit.

use std::net::{SocketAddrV4, Ipv4Addr, TcpListener};
use std::io::{Read, Write};
use std::time::Duration;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Collect program arguments.
    let mut args = std::env::args();
    if args.len() != 4 {
        eprintln!("Invalid arguments. Expected usage: ./passthrough SERIAL_DEVICE BAUD_RATE TCP_PORT");
        eprintln!("Example: ./passthrough /dev/tty.usbmodemTEST1 115200 8080");
        std::process::exit(1);
    }
    let path = args.next().unwrap();
    let baud_rate = args.next().unwrap().parse::<u32>()?;
    let tcp_port: u16 = args.next().unwrap().parse()?;


    // Setup serial port.
    let mut serial = serialport::new(path, baud_rate)
        .timeout(Duration::from_millis(10))
        .open()?;

    // Setup networking.
    let addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), tcp_port);
    let listener = TcpListener::bind(addr)?;
    println!("Pass-through listening on 127.0.0.1:{tcp_port}");

    // Run the pass through.
    for stream in listener.incoming() {
        let mut stream = stream?;

        // While there is an active connection, pipe content back and forth
        // between the TCP stream and the serial port.
        loop {
            // Stream to Serial
            if reader_to_writer(&mut stream, &mut serial)? == 0 {
                break;
            }
            // Serial to Stream
            if reader_to_writer(&mut serial, &mut stream)? == 0 {
                break;
            }
        }
    }
    Ok(())
}

/// Pipe available buffer contents up to 1024 bytes from a reader to a writer.
fn reader_to_writer(reader: &mut dyn Read, writer: &mut dyn Write) -> std::io::Result<usize> {
    let mut buf: [u8; 1024] = [0; 1024];
    let bytes_to_send = reader.read(&mut buf)?;
    if bytes_to_send == 0 {
        // An empty buffer indicates an EOF.
        return Ok(bytes_to_send); // 0
    }
    let mut bytes_sent = 0;
    while bytes_sent != bytes_to_send {
        let buf = &mut buf[bytes_sent..bytes_to_send];
        bytes_sent += writer.write(buf)?;
    }
    writer.flush()?; // Ensure the writer pushes the buffer to hardware.
    return Ok(bytes_to_send);
}