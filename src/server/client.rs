use std::net::{TcpStream, SocketAddr};
use std::io::Read;
use std::io::Write;
use tungstenite::server::accept;
use tungstenite::{WebSocket, Message};
use super::packet::{packet_data, packet_data::PacketData, Packet};

#[derive(Debug)]
pub struct Client {
    stream: WebSocket<TcpStream>,
    address: SocketAddr
}

impl Clone for Client {
    fn clone(&self) -> Client{
        Client{
            stream: self.stream,
            address: self.address
        }
    }
}

impl Client {
    pub fn new(stream: TcpStream, address: SocketAddr) -> Self {
        Client{
            stream: accept(stream).unwrap(),
            address: address
        }
    }

    pub fn read_packet(&mut self) -> Packet {
        let message = self.stream.read_message().unwrap();
        
        Packet::new(len, raw_data)
    }

    pub fn send_message(&self, message: Message) -> tungstenite::Result<()> {
        self.stream.write_message(message)?;

        Ok(())
    }
}