mod args_helper;
mod cipher;
mod globals;
mod io_helper;
mod math;
use args_helper::*;
#[allow(unused_imports)]
use cipher::*;
#[allow(unused_imports)]
use globals::*;
use io_helper::*;
use std::env::args;
use std::fs;
use std::io;
use std::os::unix::net::UnixListener;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() -> io::Result<()> {
    let args: Vec<String> = args().collect();
    let terminate = Arc::new(AtomicBool::new(false));
    let t_clone = terminate.clone();

    match validate_args(&args) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    let socket_path = &args[1];

    ctrlc::set_handler(move || {
        println!("\nSIGINT received! Closing program...");
        t_clone.store(true, Ordering::SeqCst);
    })
    .expect("Error setting up Ctrl-C handler");

    // Remove old socket if exists
    if Path::new(socket_path).exists() {
        fs::remove_file(socket_path)?;
    }

    // Bind to path, also creates socket if none existing
    let listener = UnixListener::bind(socket_path)?;
    println!("Server is listening on {}", socket_path);

    // loop until CTRL+C received
    //      will terminate after fulfilling one last client connection
    while !terminate.load(Ordering::SeqCst) {
        let mut sock = match listener.accept() {
            Ok((socket, _addr)) => {
                println!("Accepted client connection");
                // println!("socket: {:?}, addr: {:?}", socket, addr);
                socket
            }
            Err(e) => {
                eprintln!("Error:1 {}", e);
                fs::remove_file(socket_path)?;
                std::process::exit(1);
            }
        };
        match handle_request(&mut sock) {
            Ok(()) => {}
            Err(e) => {
                eprintln!("Error:2 {}", e);
                fs::remove_file(socket_path)?;
                std::process::exit(1);
            }
        }
    }

    // Removes socket when server is done
    fs::remove_file(socket_path)?;
    Ok(())
}
