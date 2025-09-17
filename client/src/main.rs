mod args;

use args::*;
use std::env::args;
use std::io::*;

// use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
// std::env::args_os use this version of args if arguments include invalid unicode

fn main() -> Result<()> {
    // collect() turns iterator of arguments into a vector of arguments
    //      since collect() creates many different vectors, we ensure to specify the type here
    let args: Vec<String> = args().collect();

    // prints cmd line args
    // dbg!(&args);

    // check arguments
    //      Note: ? is for the error to propagate upwards
    match validate_args(&args) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    // arg at 0 index is the program path
    let msg = &args[1];
    let shift_value = &args[2];
    let socket_path = &args[3];

    let mut sock = match UnixStream::connect(socket_path) {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
    
    println!("Connected to server at {}", socket_path);

    sock.write_all(b"Hello from client!")?;

    // // Receive reply
    // let mut buffer = [0u8; 128];
    // let n = sock.read(&mut buffer)?;
    // println!("Server replied: {}", String::from_utf8_lossy(&buffer[..n]));

    // TODO: remove this
    println!(
        "msg: {}, shift_value: {}, socket_path: {}",
        msg, shift_value, socket_path
    );

    Ok(())
}
