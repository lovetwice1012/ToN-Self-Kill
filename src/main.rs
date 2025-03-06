extern crate rosc;

use rosc::{OscPacket, OscType};
use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;
use enigo::{
    Direction::{Press, Release},
    Enigo, Key, Keyboard, Settings,
};
use std::thread;
use std::time::Duration;

fn main() {

    let addr = match SocketAddrV4::from_str("127.0.0.1:9001") {
        Ok(addr) => addr,
        Err(_) => panic!("Usage {} IP:PORT", "127.0.0.1:9001"),
    };
    let sock = UdpSocket::bind(addr).unwrap();
    println!("Listening to {}", addr);

    let mut buf = [0u8; rosc::decoder::MTU];

    loop {
        match sock.recv_from(&mut buf) {
            Ok((size, _addr)) => {
                let (_, packet) = rosc::decoder::decode_udp(&buf[..size]).unwrap();
                handle_packet(packet);
            }
            Err(e) => {
                println!("Error receiving from socket: {}", e);
                break;
            }
        }
    }
}

fn handle_packet(packet: OscPacket) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    match packet {
        OscPacket::Message(msg) => {
            if msg.addr == "/avatar/parameters/ToN_Self_Kill" {
                if let Some(OscType::Bool(value)) = msg.args.get(0) {
                    if *value {//US keyboard "=" 187  JIS keyboard "^" 222
                        enigo.key(Key::Other(187), Press).unwrap();
                        enigo.key(Key::Other(222), Press).unwrap();
                        thread::sleep(Duration::from_secs(5));
                        enigo.key(Key::Other(187), Release).unwrap();
                        enigo.key(Key::Other(222), Release).unwrap();
                    }
                }
            }
        }
        OscPacket::Bundle(bundle) => {
            for ele in bundle.content {
                handle_packet(ele)
            }
        }
    }
}
