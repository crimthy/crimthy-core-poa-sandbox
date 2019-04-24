use std::net::{
    SocketAddr,
    TcpStream,
};
use std::io::{
    Write,
    Read,
};
use std::time::Duration;
use std::str::FromStr;

use bincode::{
    deserialize,
    serialize,
};
use crate::broadcast::message::{
    Message,
    MessageLabel,
};

use crate::block::Block;

pub fn create_stream(address: &str) -> Option<TcpStream> {

    println!("Connecting to {}...", address);

    let socket_address = match SocketAddr::from_str(&address) {
        Ok(socket_address) => socket_address,
        Err(_) => {
            println!("Incorrect address format.");
            return None;
        }
    };

    let stream = match TcpStream::connect_timeout(
        &socket_address,
        Duration::from_secs(5),
    ) {
        Ok(stream) => stream,
        Err(_) => {
            println!("The peer cannot be joined.");
            return None;
        }
    };

    println!("Connected to {}.", address);

    Some(stream)
}