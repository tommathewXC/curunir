# curunir
A simple websocket Implementation in Rust. The eventual goal of this project is to have a Rust library that implements
a simple single-threaded server, and a statically defined client. 


# Usage

    mod curunir;

    fn main() {
        let client = curunir::Client::new( "localhost", 5555, "endpoint" ); // connect to ws://localhost:5555/endpoint
        client.connect();
    }
