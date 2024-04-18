use std::{
    io::{Cursor, Read},
    thread::sleep,
    time::Duration,
};

use fast_id_map::prelude::FastMap;
use mio::{
    net::{TcpListener, TcpStream},
    Events, Interest, Poll, Token,
};

fn main() {
    let initial_connection_capcaity = 128;
    const SOCKET_READ_BUFFER_SIZE: usize = 100_000;
    let mut poll = Poll::new().unwrap();
    let mut events = Events::with_capacity(initial_connection_capcaity);
    let mut listener = TcpListener::bind("0.0.0.0:25565".parse().unwrap()).unwrap();
    let addr = listener.local_addr();
    let delay = Duration::from_micros(0);

    struct Player {
        stream: TcpStream,
        index: usize,
        buf: Cursor<Vec<u8>>,
    }
    let mut connection_pool = FastMap::with_capacity(initial_connection_capcaity);

    poll.registry()
        .register(&mut listener, mio::Token(usize::MAX), Interest::READABLE)
        .unwrap();

    loop {
        poll.poll(&mut events, None).unwrap();
        for event in events.iter() {
            if event.token().0 == usize::MAX {
                let (mut stream, _addr) = listener.accept().unwrap();
                let buf = Cursor::new(Vec::from(&[0; SOCKET_READ_BUFFER_SIZE]));
                connection_pool
                    .add(|index| {
                        poll.registry()
                            .register(&mut stream, Token(index), Interest::READABLE)
                            .unwrap();
                        let value = Ok(Player { stream, index, buf });
                        println!("accepted: Player(index = {:?})", index);
                        value
                    })
                    .unwrap();
            } else {
                let value = connection_pool.get(event.token().0);
                let read_len_result = value.stream.read(value.buf.get_mut());
                if read_len_result.is_err() {
                    let index = value.index;
                    connection_pool.remove(index);
                }
                sleep(delay);
            }
        }
    }
}
