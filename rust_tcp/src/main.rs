use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str;
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0_u8; 50];
    while match stream.read(&mut buf) {
        Ok(_size) => {
            if str::from_utf8(&buf).unwrap() == "_" {
                stream.shutdown(Shutdown::Both).unwrap();
                false
            } else {
                stream.write_all("Resonse".as_bytes()).unwrap();
                stream.write_all(&buf).unwrap();
                true
            }
        }
        Err(_) => {
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:13")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    drop(listener);
    Ok(())
}
