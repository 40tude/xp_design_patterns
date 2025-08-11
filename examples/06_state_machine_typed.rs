// https://medium.com/@lucky_rydar/compile-time-state-machines-in-rust-making-invalid-states-impossible-d4de012c67e3
// https://medium.com/@syntaxSavage/the-typestate-pattern-in-rust-safer-state-machines-without-the-boilerplate-857c09d4d27b

// First define the states as type
struct Uninitialized;
struct Connected;
struct Closed;
struct HttpClient<State> {
    address: String,
    _state: State,
}

impl HttpClient<Uninitialized> {
    fn new(address: &str) -> Self {
        Self {
            address: address.into(),
            _state: Uninitialized,
        }
    }

    fn connect(self) -> HttpClient<Connected> {
        // ! <- self NOT &self
        println!("Connecting to {}...", self.address);
        HttpClient {
            address: self.address,
            _state: Connected,
        } // ! <- Returns a client with status Connected
    }
    fn status(&self) {
        println!("Status = Uninitialized");
    }
}
impl HttpClient<Connected> {
    fn send(&self, data: &str) {
        println!("Sending data: {}", data);
    }

    fn close(self) -> HttpClient<Closed> {
        // ! <- self NOT &self
        println!("Connection closed.");
        HttpClient {
            address: self.address,
            _state: Closed,
        } // ! <- Return a client with status Closed 
    }

    fn status(&self) {
        println!("Status = Connected");
    }
}

impl HttpClient<Closed> {
    fn status(&self) {
        println!("Status = Closed");
    }
}
fn main() {
    let client = HttpClient::new("http://localhost:8080");
    client.status();

    let connected_client = client.connect();
    connected_client.status();

    connected_client.send("Hello, world!");

    let closed_client = connected_client.close();
    closed_client.status();

    // closed_client.send("oops"); // Does NOT compile : method not found in `HttpClient<Closed>`
}
