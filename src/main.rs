use std::net;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

mod server;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = net::TcpListener::bind("127.0.0.1:7878")?;
    let pool = server::ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream?;
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    Ok(())
}

fn handle_connection(mut stream: net::TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get_msg = b"GET / HTTP/1.1\r\n";
    let sleep_route = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get_msg) {
        ("HTTP/1.1 200 OK", "frontend/index.html")
    } else if buffer.starts_with(sleep_route) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "frontend/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "frontend/404.html")
    };

    let contents = std::fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
