use std::io::{Cursor, Read};

use fast_id_map::prelude::FastMap;
use mio::{
    net::{TcpListener, TcpStream},
    Events, Interest, Poll, Token,
};

fn main() {
    const INITIAL_CONNECTION_CAPCAITY: usize = 128;
    const SOCKET_READ_BUFFER_SIZE: usize = 100_000;
    const SERVER_PORT: u16 = 443;

    let mut poll = Poll::new().unwrap();
    let mut events = Events::with_capacity(INITIAL_CONNECTION_CAPCAITY);
    let mut listener =
        TcpListener::bind(format!("0.0.0.0:{SERVER_PORT}").parse().unwrap()).unwrap();

    struct Player {
        stream: TcpStream,
        index: usize,
        buf: Cursor<Box<[u8]>>,
    }
    let mut connection_pool = FastMap::with_capacity(INITIAL_CONNECTION_CAPCAITY);

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
                        let value = Ok(Player {
                            stream,
                            index,
                            buf: Cursor::new(Box::new([0; 100])),
                        });
                        println!("accepted: Player(index = {:?})", index);
                        value
                    })
                    .unwrap();
            } else {
                let value = connection_pool.get(event.token().0);
                let read_len_result = value.stream.read(value.buf.get_mut());
                if read_len_result.is_err() || read_len_result.unwrap() == 0 {
                    let index = value.index;
                    connection_pool.remove(index);
                    println!("disconnected: Player(index = {:?})", index);
                }
            }
        }
    }
}
