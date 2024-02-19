mod client;
mod server;
use core::panic;
use std::net::{TcpListener, Ipv4Addr, SocketAddrV4, TcpStream, Shutdown};
use std::io::{stdin, Write};
use crate::server::Server;
use crate::client::Client;


const ADDR: Ipv4Addr = Ipv4Addr::new(192, 168, 0, 116);
const PORT: u16 = 8001;

fn main() {

    run_client();
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

