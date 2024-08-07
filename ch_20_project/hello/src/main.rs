use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use hello::ThreadPool;

// 127.0.0.1:7878
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // bind is like new function, 
    let pool = ThreadPool::new(2);

    for stream in listener.incoming().take(2) { // incoming returns an iterator of connection attempts
        let stream = stream.unwrap();
        pool.execute(|| { // rather than thread::spawn
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream); // BufReader implement std::io::BufRead trait which provides lines method
    // let http_request: Vec<_> = buf_reader
    //     .lines() // returns Result<String, std::io::Error>
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..]{
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}

// http requests have the following format where CRLF is carriage return and line fieed
// Method Request-URI HTTP-Version CRLF -> GET / HTTP/1.1
// headers CRLF -> Host onwards
// message-body -> Get requests have no body

// response format
// HTTP-Version Status-Code Reason-Phrase CRLF
// headers CRLF
// message-body