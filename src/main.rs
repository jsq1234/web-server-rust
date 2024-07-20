use std::{
    fs, io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}
};

use web_server_rust::ThreadPool;
fn main() {
    let listener = match TcpListener::bind("127.0.0.1:8888") {
        Ok(x) => x,
        Err(e) => panic!("Error in binding to localhost: {}", e),
    };

    let thread_pool = ThreadPool::new(8);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread_pool.execute(|| {
            if let Err(e) = handle_connection(stream) {
                eprintln!("Error: {e}");
            }
        });
    }
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()>{
    let reader = BufReader::new(&stream);
    let first_line = reader.lines().next().unwrap().unwrap();

    let (html, status_line) = if first_line == "GET / HTTP/1.1" {
        (fs::read_to_string("hello.html")?, "HTTP/1.1 200 OK\r\n")
    }else {
        (fs::read_to_string("404.html")?, "HTTP/1.1 404 Not Found\r\n")
    }; 

    stream.write_all(format!("{}Content-Length: {}\r\n\r\n{}", status_line, html.len(), html).as_bytes())?;

    Ok(())
}
