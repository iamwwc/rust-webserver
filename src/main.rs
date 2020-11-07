use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::{fs, thread};
use webserver::ThreadPoll;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let poll = ThreadPoll::new(4);
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        // stream.set_nonblocking(true);
        poll.execute(move||{
            handle_connection(stream)
        })
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n","hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n","404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}",status_line,contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}