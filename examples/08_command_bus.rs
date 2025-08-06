// cargo run --example 08_command_bus

// In complex systems, you often need to decouple the “what” from the “how.” That’s where Command Bus shines.
// You define
//      commands
//      handlers
//      and a bus that routes them

// Example : Simple Command Dispatcher

// Use cases : CQRS (Command Query Responsibility Segregation), Event Sourcing, Microservices
// Event processors, Task schedulers, Command/event bridges

// Why it scale
// Forces separation of input (command) and execution (handler)
// Easy to unit test
// Works beautifully with async, queues, retries
// Clear audit trails when paired with event logs

// -----------------------------------------------------
// -----------------------------------------------------
// -----------------------------------------------------
// -----------------------------------------------------
// -----------------------------------------------------
// -----------------------------------------------------
// -----------------------------------------------------
// -----------------------------------------------------

// What does this code implement?
// This is a generic command-dispatching system. It decouples:
//      The command: what you want to do (e.g., create a user),
//      The handler: the logic that knows how to do it.
// This is a simplified form of the Command pattern with generic dispatching, which is similar in spirit to how command buses work in larger systems.

// What is this useful for?
// This pattern shines in decoupled and scalable architectures, such as:

// Use cases:
// Command Buses in backends:
//      You define many different commands: CreateUser, DeleteUser, SendEmail, etc.
//      You register handlers for each one.
//      You centralize dispatching through a generic dispatch() or CommandBus.
// Testing and mocking:
//      Each command and handler can be tested in isolation.
//      Handlers can be replaced or mocked easily (e.g., in unit tests).
// Middleware-style pipelines:
//      You could intercept dispatch() to log, validate, or authorize commands before handling them.
// Plugin or event-driven systems:
//      Commands act like messages; handlers can be swapped in or out.

// Why use generics?
// This design makes the dispatch() function work with any command and handler combination, as long as they follow the Command and Handler traits. It's highly extensible.

// -----------------------------------------------------
// -----------------------------------------------------
// -----------------------------------------------------
// -----------------------------------------------------
// -----------------------------------------------------
// -----------------------------------------------------
// -----------------------------------------------------
// -----------------------------------------------------

// Trait that represents a Command.
// Each command must specify what kind of output (result) it returns.
pub trait Command {
    type Output;
}

// A trait for something that can handle a Command of type `C`.
// `C: Command` ensures that the command follows the Command trait.
pub trait Handler<C: Command> {
    fn handle(&self, cmd: C) -> C::Output;
}

// Example command: "Create a user" with a name.
struct CreateUser {
    pub name: String,
}

// This command, when handled, produces a String (the Output type).
impl Command for CreateUser {
    type Output = String;
}

// Handler for the CreateUser command.
struct CreateUserHandler;

// This handler knows how to handle a `CreateUser` command.
impl Handler<CreateUser> for CreateUserHandler {
    fn handle(&self, cmd: CreateUser) -> String {
        format!("Created user: {}", cmd.name)
    }
}

// This function is the "Command Bus" or Dispatcher.
// It takes any command and a matching handler, and invokes the handler.
fn dispatch<C: Command, H: Handler<C>>(cmd: C, handler: H) -> C::Output {
    handler.handle(cmd)
}

fn main() {
    // let result = dispatch(CreateUser { name: String::from("Alice") }, CreateUserHandler);
    let result = dispatch(CreateUser { name: "Alice".into() }, CreateUserHandler);
    println!("{result}"); // Output: Created user: Alice
}
