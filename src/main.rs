mod curunir {

    use std::io::{Write, Read};
    use std::net::TcpStream;
    use std::str;

    pub struct Client{
        url: String
    }

    impl Client {

        pub fn new(address: &str, port: u16 ) -> Client {
            Client{
                url: format!( "{}:{}", address, port )
            }
        }

        pub fn on_message( message: &String ){
            println!( "{}", message );
        }

        pub fn connect( &self ) {
            match TcpStream::connect( self.url.to_string() ) {
                Ok(mut stream) => {
                    let headers = [
                        "GET /ws HTTP/1.1",
                        "Host: localhost:5555",
                        "Upgrade: websocket",
                        "Connection: Upgrade",
                        "Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==",
                        "Origin: chrome-extension://pfdhoblngboilpfeibdedpjgfnlcodoo",
                        "Sec-WebSocket-Protocol: chat, superchat",
                        "Sec-WebSocket-Version: 13",
                        "Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==",
                        "Sec-WebSocket-Extensions: permessage-deflate; client_max_window_bits",
                        "Accept-Encoding: gzip, deflate, br",
                        "Pragma: no-cache",
                        "Cache-Control: no-cache",
                        "\r\n"
                    ];
                    let response = headers.join("\r\n").to_string().into_bytes();
                    match stream.write( &response ){
                        Ok(_) => {
                            let mut handshaken = false;
                            loop {
                                let mut buffer = [0; 100000 ];
                                let resp =stream.read( &mut buffer );
                                match resp {
                                    Ok(n) => {
                                        if n == 0 {
                                            break;
                                        }
                                        if !handshaken {
                                            let s = match str::from_utf8( &buffer ) {
                                                Ok(v) => v,
                                                Err(e) => panic!("Handshake Failed! Invalid UTF-8 sequence: {}", e),
                                            };
                                            println!("{}", s );
                                            handshaken = true;
                                        }else{
                                            let mut message = String::new();
                                            println!( "Byte 1 {}, Byte 2 {}", buffer[0], buffer[1] );
                                            for i in 2..n {
                                                message = format!( "{}{}", message, buffer[i] as char );
                                            }
                                            Client::on_message( &message );
                                        }
                                    }
                                    Err(e2) => {
                                        println!("Socket error {}", e2 );
                                    }
                                }
                            }
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
    }
}


fn main() {
    let client = curunir::Client::new( "localhost", 5555 );
    client.connect();
}
