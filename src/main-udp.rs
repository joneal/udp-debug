
// use std::net::UdpSocket;

// fn main(){
   
//     let mut data: Vec<u8> = Vec::new();
//     data.push(0x41);
//     data.push(0x42);
//     data.push(0x43);

//     let socket = UdpSocket::bind("0.0.0.0:0").expect("could not bind socket to address:port [0.0.0.0:0]");
//     socket.set_multicast_loop_v4(false).expect("could not disable multicast loop");
//     socket.set_broadcast(true).expect("could not set broadcast flag");

//     println!("Broadcasting...");
//     socket.send_to(&data, "255.255.255.255:9999").expect("could not broadcast message");
// }