mod client;
mod server;

use std::{fs, io::Cursor, thread};

use binrw::{binrw, BinRead, BinWrite};
use client::client::Client;
use server::server::Server;

#[binrw]
#[brw(little)]
#[derive(Debug)]
struct Data {
    // 130 == player data?
    // 005 == handshake?
    kind: u8,
    client_tick: u16,
    server_tick: u16,
    session_id: u32,
}

fn dump(data: &[u8]) -> std::io::Result<()> {
    fs::write("out.bin", data)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Client::new("127.0.0.1:6666", "127.0.0.1:6667")?;
    let server = Server::new("127.0.0.1:6667")?;

    thread::spawn(move || -> Result<(), std::io::Error> {
        server.listen()?;
        Ok(())
    });

    let datagrams = fs::read_to_string("data/output.mayo")?;

    for line in datagrams.split("\n") {
        let mut data = Vec::<u8>::new();

        for hex in line.split(":") {
            if hex.len() == 0 {
                continue;
            }
            data.push(u8::from_str_radix(hex, 16)?);
        }

        let mut cursor = Cursor::new(data);
        let datagram = Data::read(&mut cursor)?;

        dbg!(datagram);
    }

    let mut buffer = Cursor::new([0u8; 16]);

    // data.write(&mut buffer)?;

    let bytes = client.write(buffer.get_ref())?;

    dump(buffer.get_ref())?;

    println!("wrote {:?} bytes", bytes);

    Ok(())
}
