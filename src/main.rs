mod client;
mod server;
use core::panic;
use std::net::{TcpListener, Ipv4Addr, SocketAddrV4, TcpStream, Shutdown};
use std::io::{stdin, Write, Read};
use crate::server::Server;
use crate::client::Client;


const ADDR: Ipv4Addr = Ipv4Addr::new(192, 168, 0, 116);
const PORT: u16 = 8001;

const IS_SERVER: bool = true;

fn main() {
    if IS_SERVER {
        run_server()
    }
    else {
        run_client();
    }
    
}

fn run_client() {
    let mut user_input: String = String::new();

    println!("please enter your name: ");
    match stdin().read_line(&mut user_input) {
        Ok(_num_bytes) => {
            println!("We have successfully read: {}", &user_input);
        }
        Err(_error) => {
            user_input = String::from("TstCse failed");
            println!("Input was misread");
        }
    }

    let stream_result = TcpStream::connect(SocketAddrV4::new(ADDR, PORT));

    match stream_result {
     Ok (mut stream) => {
        println!("Connected to the server on {:?}", stream.peer_addr().unwrap());

        // let message = args().nth(1).expect("Please provide message!");
        match user_input.as_str() {
            "#END#" => stream.shutdown(Shutdown::Both).expect("Shutdown Failed!"),
            &_ => {panic!("aghhhhhhhh");}
        }
        
        match stream.write(&user_input.into_bytes()) {
                    Ok(_) => {print!("SENT!");}
                    Err(_) => {
                        println!("user input had an invalid value");
                    }
                }
                //stream.read(&mut [0; 128])?;
        }
        
        Err(_e) => {
            println!("Couldn't connect to server...");
        }
    }
    
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
                                
                                while match stream.read(&mut data) {
                                    Ok(size) => {
                                        match stream.write(&data[0..size]) {
                                            Ok(amnt_read) => {println!("Read: {} bytes", amnt_read);}
                                            Err(error) => {
                                                println!("failed to recieve message due to: {}", error);
                                            }
                                        }
                                        true
                                    }
                                    Err(error) => {
                                        println!("failed to read stream into data because of: {}", error);
                                        false
                                    }
                                } {}
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
            println!("could not establish a listener on: {:?}, {:?}", PORT, ADDR);
            panic!("{}", error);
        }
    }

}

#[cfg(test)]
     mod tests {
         #[test]
         fn simple_unlock() {
             use crate::client::Client;
             use crate::server::Server;

             let mut s1: Server = Server::new(1);
             let mut client: Client = Client::new(1);
         
             assert!(s1.check_transmission(client.get_transmission()));
         }
        
        #[test]
         fn repeated_unheard() {
            use crate::client::Client;
            use crate::server::Server;

            let mut s1: Server = Server::new(1);
            let mut client: Client = Client::new(1);
            client.get_transmission();
            client.get_transmission();
            client.get_transmission();
            client.get_transmission();
            assert!(s1.check_transmission(client.get_transmission()));
        }
    }

    #[test]
    fn testConnection() {
        use std::net::TcpListener;
        

    }

