use std::thread;
use std::{
    io::{BufWriter, Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
};

fn handle_client(mut stream: TcpStream) {
    let mut buf: [u8; 50] = [0u8; 50];
    let mut buf_writer: BufWriter<Vec<u8>> = BufWriter::new(Vec::new());
    while match stream.read(&mut buf) {
        Ok(_size) => {
            let _ = buf_writer.write_fmt(format_args!("NEW BUFFER DATA:"));
            stream.write_all(buf_writer.buffer()).unwrap();
            stream.write_all(&buf).unwrap();
            let _ = stream.flush();
            let _ = buf_writer.flush();
            true
        }
        Err(_) => {
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() -> std::io::Result<()> {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:13")?;

    listener.incoming().for_each(|stream| match stream {
        Ok(stream) => {
            thread::spawn(move || {
                handle_client(stream);
            });
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    });
    drop(listener);
    Ok(())
}
