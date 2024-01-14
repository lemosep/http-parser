use std::{
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
};
fn main() {
    const LOCAL_ADDR: &str = "127.0.0.1:7878";

    let listener = TcpListener::bind(LOCAL_ADDR).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_request(stream);
    }
}

fn handle_request(stream: TcpStream) {
    let reader = BufReader::new(&stream);

    let http_req: Vec<String> = reader
        .lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("{:#?}", http_req);
}
