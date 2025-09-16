mod cipher;
mod globals;
mod math;

use cipher::*;
use globals::*;
use std::fs;
use std::io;
use std::os::unix::net::UnixListener;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() -> io::Result<()> {
    let terminate = Arc::new(AtomicBool::new(false));
    let socket_path = "temp/socket";
    let t_clone = terminate.clone();

    ctrlc::set_handler(move || {
        println!("\nSIGINT received! Closing program...");
        t_clone.store(true, Ordering::SeqCst);
    })
    .expect("Error setting up Ctrl-C handler");

    let msg: &str = "If he had anything confidential to say, he wrote it in cipher, that is, by so changing the order of the letters of the alphabet, that not a word could be made out.";
    println!("{} - {}", UP_MIN_INDEX, UP_MAX_INDEX);
    encrypt(msg, -529);

    if Path::new(socket_path).exists() {
        fs::remove_file(socket_path)?;
    }

    let listener = UnixListener::bind(socket_path)?;
    println!("Server is listening on {}", socket_path);

    // loop until CTRL+C received
    while !terminate.load(Ordering::SeqCst) {
        match listener.accept() {
            Ok((socket, addr)) => {
                println!("Accepted client connection");
                println!("socket: {:?}, addr: {:?}", socket, addr);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    fs::remove_file(socket_path)?;
    Ok(())
}
