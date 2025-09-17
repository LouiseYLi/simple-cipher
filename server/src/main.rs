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

    // let msg: &str = "If he had anything confidential to say, he wrote it in cipher, that is, by so changing the order of the letters of the alphabet, that not a word could be made out.";
    // println!("{} - {}", UP_MIN_INDEX, UP_MAX_INDEX);
    // encrypt(msg, -529);

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

// fn validate_args(args: &[String]) -> Result<(), String> {
//     validate_length(args.len() as i32)?;
//     validate_path(&args[1])?;

//     Ok(())
// }

// fn validate_length(vector_length: i32) -> Result<(), String> {
//     const MAX_ARGS: i32 = 2;
//     if vector_length != MAX_ARGS {
//         return Err(format!(
//             "Invalid number of args... Expected 4, actual {}",
//             vector_length
//         ));
//     }

//     Ok(())
// }

// fn validate_path(path_str: &str) -> Result<(), String> {
//     let path = Path::new(path_str);
//     if !(path.exists()) {
//         return Err(format!("Path {} does not exist", path_str));
//     }

//     Ok(())
// }
