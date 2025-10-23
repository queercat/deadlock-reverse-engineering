use std::net::UdpSocket;

pub struct Client {
    stream: UdpSocket,
}

impl Client {
    pub fn new(from: &'static str, to: &'static str) -> Result<Self, std::io::Error> {
        let socket = UdpSocket::bind(from)?;

        socket.connect(to)?;

        Ok(Client { stream: socket })
    }

    pub fn write(&mut self, buffer: &[u8]) -> Result<usize, std::io::Error> {
        Ok(self.stream.send(buffer)?)
    }
}
