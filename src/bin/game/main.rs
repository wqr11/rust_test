use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

fn handle_connection(mut conn: TcpStream) -> () {
    println!("handling connection");

    let mut buf = String::new();

    conn.read_to_string(&mut buf).unwrap();

    print!("{}", buf);
}

fn main() {
    let tcp = TcpListener::bind("0.0.0.0:9999").expect("Counldn't bind to port ::9999");

    println!("Running at 0.0.0.0:9999");

    for client in tcp.incoming() {
        handle_connection(client.unwrap());
    }
}
