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
    kind: u8,
    client_tick: u16,
    server_tick: u16,
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

    let datagram = fs::read("data/datagram.bin")?;

    let data = Data::read(&mut Cursor::new(datagram))?;

    dbg!(&data);

    let mut buffer = Cursor::new([0u8; 16]);

    data.write(&mut buffer)?;

    let bytes = client.write(buffer.get_ref())?;

    dump(buffer.get_ref())?;

    println!("wrote {:?} bytes", bytes);

    Ok(())
}
