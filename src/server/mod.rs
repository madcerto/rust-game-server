use std::net::{TcpStream, TcpListener, SocketAddr};
use std::collections::HashMap;
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tungstenite::Message;

pub mod client;
use client::{Client};
pub mod packet;
use packet::{packet_type::PacketType, Packet};

pub trait EventListener {
    fn on_packet_recv(&self, client: &Client, message: Message);
    fn on_loop(&self, delta: usize);
    fn send_packet(&self, client: &Client, message: Message) -> tungstenite::Result<()>{
        client.send_message(message)?;

        Ok(())
    }
}

pub struct Server {
    connected_clients: Vec<Client>,
    event_subscribers: HashMap<PacketType, Vec<Box<dyn EventListener>>>
}

unsafe impl Send for Server {}

impl Server {
    pub fn new() -> Self {
        Server{
            connected_clients: vec![],
            event_subscribers: HashMap::new()
        }
    }

    pub fn register_event_listener<E: EventListener + 'static>(&mut self, packet_type: PacketType, listener: E) {
        // Get event subscribers of specific packet
        let packet_subscribers: &mut Vec<Box<dyn EventListener>> = self.event_subscribers.get_mut(&packet_type).unwrap();
        // Insert new subscriber
        packet_subscribers.push(Box::new(listener));
        // self.event_subscribers.insert(packet_type, packet_subscribers);
    }

    pub fn run(self) -> std::io::Result<()> {
        // Open socket
        let socket = TcpListener::bind("127.0.0.1:34254")?;

        // Create mutexes
        let self_mutex = Arc::new(Mutex::new(self));

        // Spawn thread that accepts new connections
        Self::accept_connections(self_mutex.clone(), socket);

        // Process loop
        let mut delta: usize = 0;
        let mut cur_time: Instant;
        loop {
            cur_time = Instant::now();

            let self_ = self_mutex.lock().unwrap();
            let subscribers_hm = &self_.event_subscribers;

            for subscribers in subscribers_hm {
                for subscriber in subscribers.1 {
                    subscriber.on_loop(delta);
                }
            }

            delta = cur_time.elapsed().as_secs_f32() as usize;
        }
    }

    fn accept_connections(self_mutex: Arc<Mutex<Self>>, socket: TcpListener) {
        thread::spawn(move || {
            loop {
                // Get connection from socket
                let result = socket.accept();

                if result.is_err() {
                    println!("Could not complete requested connection: {}", result.unwrap_err());
                    continue
                }

                let (client_stream, client_address) = result.unwrap();
                client_stream.set_read_timeout(Some(Duration::from_secs(90))).unwrap();

                Self::add_client(self_mutex.clone(), client_stream, client_address);
            }
        });
    }

    fn add_client(self_mutex: Arc<Mutex<Self>>, client_stream: TcpStream, client_address: SocketAddr) {
        let client = Client::new(client_stream, client_address);

        // Spawn thread that gets packets from client and informs event subscribers
        Self::recieve_packets(self_mutex.clone(), client.clone());

        // Lock self and push client into vec
        let mut self_ = self_mutex.lock().unwrap();
        self_.connected_clients.push(client);
    }

    fn recieve_packets(self_mutex: Arc<Mutex<Self>>, mut client: Client) {
        thread::spawn(move || {
            loop {
                // Read packet from TCP stream
                let packet = client.read_packet();
                Self::trigger_event(self_mutex.clone(), client.clone(), packet)
            }
        });
    }

    fn trigger_event(self_mutex: Arc<Mutex<Self>>, client: Client, packet: Packet) {
        let self_ = self_mutex.lock().unwrap();
        let subscribers = self_.event_subscribers.get(&packet.packet_type).unwrap();

        for subscriber in subscribers {
            subscriber.on_packet_recv(&client, packet.clone());
        }
    }
}