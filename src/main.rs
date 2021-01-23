#[allow(dead_code)]
use std::net::UdpSocket;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};

const MAX_MTU: usize = 1000;
const BROADCAST_PORT: u16 = 9999;

#[derive(Debug)]
struct UdpDebug {
    src_addr: SocketAddr,
    dest_addr: SocketAddr,
    socket: Option<UdpSocket>,
    is_ready: bool,
    cr_lf: [u8; 2],
}

// Methods
impl UdpDebug {
    fn new() -> UdpDebug {
        UdpDebug {
            src_addr: SocketAddr::from(([0, 0, 0, 0], 0)),
            dest_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(255,255,255,255)), BROADCAST_PORT),
            socket: None,
            is_ready: false,
            cr_lf: [0x0D, 0x0A],
        }
    }
}

impl UdpDebug {
    fn broadcast(&mut self, msg: &str) {
        // Initialize the UDP socket with first call to Broadcast
        if !self.is_ready {
            self.socket = Some(
                UdpSocket::bind(self.src_addr)
                    .expect("Could not bind socket to address:port [0.0.0.0:0]"),
            );
            self.socket
                .as_ref()
                .unwrap()
                .set_multicast_loop_v4(false)
                .expect("Could not disable multicast loop");
            self.socket
                .as_ref()
                .unwrap()
                .set_broadcast(true)
                .expect("Could not set broadcast flag");
            self.is_ready = true;
        }

        // Broadcast the message to the destination UDP port.
        let buffer = msg.as_bytes();

        if buffer.len() > MAX_MTU {

        }

        self.socket
            .as_ref()
            .unwrap()
            .send_to(&buffer, self.dest_addr)
            .expect("Could not broadcast message");
        self.socket
            .as_ref()
            .unwrap()
            .send_to(&self.cr_lf, self.dest_addr)
            .expect("Could not broadcast CRLF");
    }
}

fn main() {
    let mut logger: UdpDebug = UdpDebug::new();

    logger.broadcast("This is a test.");
}
