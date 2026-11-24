use std::{
    io::Read,
    net::{TcpListener, TcpStream},
    thread,
};

fn handle_connection(mut conn: TcpStream) -> () {
    println!("handling connection");

    let mut buf = String::new();

    conn.read_to_string(&mut buf).unwrap();

    print!("{}", buf);
}

fn main() {
    let tcp = TcpListener::bind("0.0.0.0:9999").unwrap();
    let max_threads = thread::available_parallelism().unwrap();

    for t in 0_usize..max_threads {}

    println!("Running at 0.0.0.0:9999");

    for client in tcp.incoming() {
        handle_connection(client.unwrap());
    }
}
