mod server;
use core::panic;
use std::net::{TcpListener, Ipv4Addr, SocketAddrV4, TcpStream, Shutdown};
use std::io::{stdin, Write, Read};
use crate::server::Server;


const ADDR: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 9);
const PORT: u16 = 8008;

fn main() {
    run_server();
}

fn run_server() {
 // don't use multiple threads here as you should only make one transaction at a time anyways
 // could make a queue of requests if neccessary
    match TcpListener::bind(SocketAddrV4::new(ADDR, PORT)) {
        Ok(listener) => {
            for stream in listener.incoming() {
                match stream {
                    Ok(mut stream) => {
                        match stream.peer_addr() {
                            Ok(addr) => {
                                println!("Connection has been established with: {}", addr);
                                
                                // handle server logic here
                                let mut data = [0 as u8;8];
                                match stream.read(&mut data) {
                                    Ok(size) => {
                                        if size != 0 {
                                        match stream.write(&data[0..size]) {
                                            Ok(_amnt_read) => {
                                                println!("State of buffer: {:?}", &data);
                                            }
                                            Err(error) => {
                                                println!("failed to recieve message due to: {}", error);
                                                data = [0 as u8; 8];
                                            }
                                        }
                                        }
                                    }
                                    Err(error) => {
                                        println!("failed to read stream into data because of: {}", error);
                                        match stream.shutdown(Shutdown::Both) {
                                            Ok(()) => {
                                                println!("Shutting down server and client");
                                            }
                                            Err(error_sd) => {
                                                println!("Could not shut down both server and client because of: {}", error_sd);
                                                match stream.shutdown(Shutdown::Read) {
                                                    Ok(()) => {
                                                        println!("Successfully shutdown the server but client may or may not be active");
                                                    }
                                                    Err(error_sd2) => {
                                                        println!("Could not shut down either anything because of: {} resulting to panicking", error_sd2);
                                                        panic!("see logs");
                                                    }
                                                }
                                            }

                                        }
                                    }
                                }
                            }
                            Err(_e) => {
                                println!("Connection has been established but not sure who with");
                            }
                        }
                    }
                    Err(err) => {
                        println!("Connection failed due to: {:?}", err);
                    }
                }
            }
        }

        Err(error) => {
            println!("could not establish a listener on: {:?}, {:?}", ADDR, PORT);
            panic!("{}", error);
        }
    }

}