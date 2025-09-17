mod args;
mod io_helper;

use args::*;
use io_helper::*;
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
    // let msg = &args[1];
    // let shift_value = &args[2];
    // let socket_path = &args[3];

    let buffer: String = format!(
        "{}{}{}{}{}{}",
        &args[1].len(),
        &args[1],
        &args[2].len(),
        &args[2],
        &args[3].len(),
        &args[3]
    );

    println!(
        "buffer: {}, buffer as bytes: {:?}",
        buffer,
        &buffer.as_bytes()
    );

    let mut sock = match UnixStream::connect(&args[3]) {
        Ok(stream) => {
            println!("Connected to server at {}", &args[3]);
            stream
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match write_buffer(&mut sock, &args[1]) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
    match write_buffer(&mut sock, &args[2]) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
    match write_buffer(&mut sock, &args[3]) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
    // // Writes length of the message as a 4 byte unsigned integer in BE order
    // match sock.write_u32::<BigEndian>(buffer.as_bytes().len()) {
    //     Ok(()) => {}
    //     Err(e) => {
    //         eprintln!("Error: {}", e);
    //         std::process::exit(1);
    //     }
    // }

    // match sock.write_all(buffer.as_bytes()) {
    //     Ok(()) => {}
    //     Err(e) => {
    //         eprintln!("Error: {}", e);
    //         std::process::exit(1);
    //     }
    // }

    // // Receive reply
    // let mut buffer = [0u8; 128];
    // let n = sock.read(&mut buffer)?;
    // println!("Server replied: {}", String::from_utf8_lossy(&buffer[..n]));

    // // TODO: remove this
    // println!(
    //     "msg: {}, shift_value: {}, socket_path: {}",
    //     msg, shift_value, socket_path
    // );

    Ok(())
}
