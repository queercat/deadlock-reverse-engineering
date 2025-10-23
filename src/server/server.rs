use std::net::UdpSocket;

pub struct Server {
    stream: UdpSocket,
}

impl Server {
    pub fn new(from: &'static str) -> Result<Self, std::io::Error> {
        let socket = UdpSocket::bind(from)?;


        Ok(Server { stream: socket })
    }

    pub fn listen(&self) -> Result<(), std::io::Error> {
        let mut buffer = [0u8; 16];
        self.stream.recv(&mut buffer)?;

        println!("got {:#04x?}", buffer);

        Ok(())
    }
}
