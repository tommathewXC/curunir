# curunir
A simple websocket Implementation in Rust. This project is an exercise in learning to use Rust, particularly in a networking environment. 

The eventual goal of this project is to have a Rust library that implements
a simple single-threaded server, and a statically defined client. 


# Usage

    mod curunir;

    fn main() {
        let client = curunir::Client::new( "localhost", 5555, "endpoint" ); // connect to ws://localhost:5555/endpoint
        client.connect();
    }


# Shortcomings

1. Does not have an implementation for a websocket server (yet).
2. No implementation for masking (yet).
3. Only works for small payloads ( size < 126 bytes )
4. No clean way to override the onMessage event handler for the WS client.


# Reference

The WebSocket Protocol Standard [RFC6455](https://tools.ietf.org/html/rfc6455)
