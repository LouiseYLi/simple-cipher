use byteorder::{BigEndian, WriteBytesExt};
use std::io::*;
use std::os::unix::net::UnixStream;

pub fn write_buffer(sock: &mut UnixStream, buffer: &String) -> Result<()> {
    // Writes length of the message as a 4 byte unsigned integer in BE order
    sock.write_u32::<BigEndian>(buffer.len() as u32)?;
    // // Writes buffer to socket
    sock.write_all(buffer.as_bytes())?;

    Ok(())
}

pub fn write_request(sock: &mut UnixStream, args: &[String]) -> Result<()> {
    write_buffer(sock, &args[1])?;
    write_buffer(sock, &args[2])?;
    write_buffer(sock, &args[3])?;

    Ok(())
}
