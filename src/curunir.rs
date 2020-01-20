use std::io::{Write, Read};
use std::net::TcpStream;
use std::str;

pub struct Client{
    address: String,
    port: u16,
    endpoint: String,
    key: String
}

impl Client {

    pub fn new(address: &str, port: u16, endpoint: &str ) -> Client {
        Client{
            address: format!( "{}", address ),
            port: port,
            endpoint: format!("{}", endpoint ),
            key: String::from("dGhlIHNhbXBsZSBub25jZQ==")
        }
    }

    /**
     * Send Message
     * ---
     * 
     * Send a string message over the websocket. Under work.
     * 
     * @returns     Resut<size,Error>   A result object
     */
    pub fn send( stream : &mut std::net::TcpStream, message: String ) -> std::result::Result< usize, std::io::Error > {
        let mut bytes = message.into_bytes();
        bytes.reverse();
        let payload = Client::frame_data( bytes );
        let response = stream.write( &payload );
        response
    }


    /**
     * The handler for a message event
     * ---
     * 
     * Handles a message event on the socket
     * 
     * @params  stream  A tcp stream mutable reference, to write to if need be
     * @params  message The Incoming message from the WebSocket's TCP buffer
     */
    pub fn on_message( stream : &mut std::net::TcpStream, message: &String ){
        if message.len() > 0 {
            println!("Server says: '{}'", message );
        }
        let outbound = String::from("h");
        match Client::send( stream, outbound ) {
            Ok(r) => {
                println!("Send {} bytes", r );
            }
            Err(er) => {
                println!("Send error {}" , er );
            }
        }
    }

    /**
     * Connect
     * ---
     * 
     * Connected to the specificed WebSocket server, and fire the message handler(s)
     * 
     */
    pub fn connect( &self ) {
        let url = format!( "{}:{}", self.address, self.port );
        match TcpStream::connect( url ) {
            Ok(mut stream) => {
                let shake = Client::handshake( &mut stream, &self.address, &self.port, &self.endpoint, &self.key );
                match shake {
                    Ok(_) => {
                        Client::start( &mut stream );
                    }
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Failed to connect: {}", e);
            }
        }
    }

    fn start( stream : &mut std::net::TcpStream ){
        let mut handshaken = false;
        loop {
            let mut buffer = [0; 100000 ];
            let resp =stream.read( &mut buffer );
            match resp {
                Ok(n) => {
                    if n > 0 {
                        if !handshaken {
                            let s = match str::from_utf8( &buffer ) {
                                Ok(v) => v,
                                Err(e) => panic!("Handshake Failed! Invalid UTF-8 sequence: {}", e),
                            };
                            println!("{}", s );
                            handshaken = true;
                        }else{
                            let mut message = String::new();
                            for i in 2..n {
                                message = format!( "{}{}", message, buffer[i] as char );
                            }
                            Client::on_message( stream, &message );
                        }
                    }
                }
                Err(e2) => {
                    println!("Socket error {}", e2 );
                }
            }
        }
    }

    fn handshake( stream : &mut std::net::TcpStream, 
        address: &String, 
        port: &u16, 
        endpoint: &String,
        key: &String ) -> std::result::Result< usize, std::io::Error> {
            
        let mut get = format!("GET /{} HTTP/1.1\n", endpoint );
        get = format!("{}Host: {}:{}\n", get, address, port );
        get = format!("{}Sec-WebSocket-Key: {}\n", get, key );
        let headers = [
            "Upgrade: websocket",
            "Connection: Upgrade",
            "Origin: CurunirClient",
            "Sec-WebSocket-Protocol: chat, superchat",
            "Sec-WebSocket-Version: 13",
            "Sec-WebSocket-Extensions: permessage-deflate; client_max_window_bits",
            "Accept-Encoding: gzip, deflate, br",
            "Pragma: no-cache",
            "Cache-Control: no-cache",
            "\r\n"
        ];
        get = format!("{}{}", get, headers.join("\r\n").to_string() );
        let response = get.into_bytes();
        stream.write( &response )
    }

    fn frame_data( raw: Vec<u8> ) -> Vec<u8> {
        let mut paylod : Vec<u8> = vec![  0x81, 0x81, 0x8e, 0xbf, 0xa3, 0x3e  ];
        // let mut paylod : Vec<u8> = vec![  0x81, 0x87 ];
        // if raw.len() <= 125 {
        //     paylod.extend(vec![ raw.len() as u8, 0x70, 0xff, 0x80  ]);
        // }
        paylod.extend( raw );
        paylod
    }
}