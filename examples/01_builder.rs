// cargo run --example 01_builder

// Builders play nicely with ownership, immutability, and compile-time guarantees.
// Rust lacks default function arguments — Builder is often the cleanest way to configure complex structs.

#[derive(Debug)]
pub struct User {
    name: String,
    age: u32,
    email: Option<String>,
}

pub struct UserBuilder {
    name: String,
    age: u32,
    email: Option<String>,
}
impl UserBuilder {
    pub fn new(name: String, age: u32) -> Self {
        Self { name, age, email: None }
    }
    pub fn email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }
    pub fn build(self) -> User {
        User {
            name: self.name,
            age: self.age,
            email: self.email,
        }
    }
}

fn main() {
    // Create a user without email
    let user1 = UserBuilder::new("Alice".to_string(), 30).build();
    println!("User without email: {user1:?}");
    let (_name1, _age1, _email1) = (user1.name, user1.age, user1.email);

    // Create a user with email
    let user2 = UserBuilder::new("Bob".to_string(), 25).email("bob@example.com".to_string()).build();
    println!("User with email: {user2:?}");
    dbg!("User with email: {:?}", user2);
}
