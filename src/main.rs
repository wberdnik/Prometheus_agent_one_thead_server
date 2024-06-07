mod my_reader;
mod service;
mod config;

use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use crate::service::service::HttpService;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Can`t create TCP Socket");

    let mut a_service = service::service::HttpService::new();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, &mut a_service);
    }
}


fn handle_connection(mut stream: TcpStream, http_service: &mut HttpService) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    match http_service.run(http_request) {
        Ok(contents) => {
            let length = contents.len();
            stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\nContent-Type: application/openmetrics-text; version=1.0.0; charset=utf-8\r\n\r\n{contents}").as_bytes()).expect("Error write to socket");
        }
        Err(m) => { stream.write_all(format!("HTTP/1.1 {m}\r\n\r\n").as_bytes()).expect("Error write to socket"); }
    };
}
