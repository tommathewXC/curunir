mod curunir;

fn main() {
    let client = curunir::Client::new( "localhost", 5555, "ws" );
    client.connect();
}
