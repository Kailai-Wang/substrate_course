use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

/// main function
fn main() {    
    // &str for server address (host:port) and print the listening info
    let server_address = "127.0.0.1:2333";
    println!("TCP server listening on {}", server_address);

    // create a tcp listener using server_address
    // use pattern matching to handle the potential error
    match TcpListener::bind(server_address) {
        // if ::bind is ok, call the handler
        Ok(listener) =>  handle_listener(listener),
        // if an errror emerges in ::bind
        Err(err) => {
            // print hte error message
            eprintln!("Error binding {} : {},  quit now", server_address, err);
            // quit the program
            std::process::exit(1);
        }
    }
}

/// handle the TCP listener
///
/// called when a server listener is successfully created
fn handle_listener(listener: TcpListener) {
    // iterate over all the incoming connections
    for stream in listener.incoming() {
        // error handling:
        // for Ok case: spawn a thread to handle the stream
        // for Err case: print error message and ignore the stream
        match stream {
            Ok(stream) => {
                // print client address, use unwrap to retrieve Ok value only
                println!("Received new connection from: {}", stream.peer_addr().unwrap());
                // spawn the thread which takes ownership of stream and handles it
                thread::spawn(move || {
                    handle_stream(stream);
                });
            }
            Err(err) => {
                eprintln!("Error incoming stream: {}", err);
            }
        }
    }

    // close the socket server
    println!("closing the listener ...");
    drop(listener);
}

/// handle the incoming stream
/// 
/// read the data from incoming stream into buffer
/// and echo the same message to the client
/// 
/// The stream connection is kept alive unless some error
/// happens or user terminates it
fn handle_stream(mut stream: TcpStream) {
    // use 1024 bytes read buffer
    let mut buffer = [0 as u8; 1024];

    // use while loop to keep the connection alive,
    // as long as no error occurs and user doesn't terminate the connection
    while match stream.read(&mut buffer) {
        Ok(size) => {
            // if client disconnects it, 0 bytes will be read
            if size == 0 {
                println!("Read 0 bytes, client might have closed the connection : {}", stream.peer_addr().unwrap());
                false
            } else {
                // echo everything
                stream.write(&buffer[0..size]).unwrap();
                true
            }
        },
        Err(_) => {
            // print error message and shutdown the stream (for both read and write)
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}

    println!("done handling stream from {}", stream.peer_addr().unwrap());
}