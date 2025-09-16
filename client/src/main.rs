use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

fn main() -> std::io::Result<()> {
    let socket_path = "temp/socket";

    let mut sock = UnixStream::connect(socket_path)?;
    println!("Connected to server at {}", socket_path);

    sock.write_all(b"Hello from client!")?;

    // Receive reply
    let mut buffer = [0u8; 128];
    let n = sock.read(&mut buffer)?;
    println!("Server replied: {}", String::from_utf8_lossy(&buffer[..n]));

    Ok(())
}
