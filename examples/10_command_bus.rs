// cargo run --example 10_command_bus

// Command Bus with more than one command
// Added middleware (here, logging)
// Added Error management (vs panic previously)

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt;

// Command Trait (base)
pub trait Command {
    type Output;
}

pub trait Handler<C: Command> {
    fn handle(&self, cmd: C) -> C::Output;
}

// Middleware - logging
trait CommandLogger {
    fn log(&self, message: &str);
}

struct ConsoleLogger;

impl CommandLogger for ConsoleLogger {
    fn log(&self, message: &str) {
        println!("[LOG] {message}");
    }
}

// Commands
#[derive(Debug)]
struct CreateUser {
    pub name: String,
}

impl Command for CreateUser {
    type Output = Result<String, String>;
}

#[derive(Debug)]
struct DeleteUser {
    pub id: u32,
}

impl Command for DeleteUser {
    type Output = Result<bool, String>;
}

// Handlers
struct CreateUserHandler {
    logger: Box<dyn CommandLogger>,
}

impl CreateUserHandler {
    pub fn new(logger: Box<dyn CommandLogger>) -> Self {
        CreateUserHandler { logger }
    }
}

impl Handler<CreateUser> for CreateUserHandler {
    fn handle(&self, cmd: CreateUser) -> Result<String, String> {
        self.logger.log(&format!("Try to delete user: {}", cmd.name));

        if cmd.name.is_empty() {
            Err("Name cannot be empty".to_string())
        } else {
            let result = format!("User created: {}", cmd.name);
            self.logger.log(&format!("Success: {result}"));
            Ok(result)
        }
    }
}

struct DeleteUserHandler {
    logger: Box<dyn CommandLogger>,
}

impl DeleteUserHandler {
    pub fn new(logger: Box<dyn CommandLogger>) -> Self {
        DeleteUserHandler { logger }
    }
}

impl Handler<DeleteUser> for DeleteUserHandler {
    fn handle(&self, cmd: DeleteUser) -> Result<bool, String> {
        self.logger.log(&format!("Try to delete user: {}", cmd.id));

        if cmd.id == 0 {
            Err("Invalid ID".to_string())
        } else {
            self.logger.log(&format!("User {} deleted", cmd.id));
            Ok(true)
        }
    }
}

// CommandBus with error mgt
struct CommandBus {
    handlers: HashMap<TypeId, Box<dyn Any>>,
    logger: Box<dyn CommandLogger>,
}

impl CommandBus {
    pub fn new(logger: Box<dyn CommandLogger>) -> Self {
        CommandBus { handlers: HashMap::new(), logger }
    }

    pub fn register<C, H>(&mut self, handler: H)
    where
        C: Command + 'static,
        H: Handler<C> + 'static,
    {
        self.handlers.insert(TypeId::of::<C>(), Box::new(handler));
        self.logger.log(&format!("Handler registered for the command {:?}", TypeId::of::<C>()));
    }

    pub fn dispatch<C, H>(&self, cmd: C) -> C::Output
    where
        C: Command + fmt::Debug + 'static,
        H: Handler<C> + 'static,
    {
        self.logger.log(&format!("Dispatching of the command: {cmd:?}"));

        let type_id = TypeId::of::<C>();
        match self.handlers.get(&type_id) {
            Some(handler) => match handler.downcast_ref::<H>() {
                Some(handler) => handler.handle(cmd),
                None => {
                    let msg = format!("Wrong handler type for the command {type_id:?}");
                    self.logger.log(&msg);
                    panic!("{}", msg)
                }
            },
            None => {
                let msg = format!("No handler registered for the command {type_id:?}");
                self.logger.log(&msg);
                panic!("{}", msg)
            }
        }
    }
}

fn main() {
    // Logger initialization
    let logger = Box::new(ConsoleLogger);

    // Command Bus initialization (with the logger)
    let mut bus = CommandBus::new(logger);

    // Registers the handlers with their own logger
    bus.register::<CreateUser, CreateUserHandler>(CreateUserHandler::new(Box::new(ConsoleLogger)));
    bus.register::<DeleteUser, DeleteUserHandler>(DeleteUserHandler::new(Box::new(ConsoleLogger)));

    // Execute commands with error management
    match bus.dispatch::<CreateUser, CreateUserHandler>(CreateUser { name: "Alice".into() }) {
        Ok(result) => println!("Result: {result}"),
        Err(e) => println!("Error: {e}"),
    }

    match bus.dispatch::<CreateUser, CreateUserHandler>(CreateUser { name: "".into() }) {
        Ok(result) => println!("Result: {result}"),
        Err(e) => println!("Error: {e}"),
    }

    match bus.dispatch::<DeleteUser, DeleteUserHandler>(DeleteUser { id: 42 }) {
        Ok(result) => println!("Deletion succeeded ? {result}"),
        Err(e) => println!("Error: {e}"),
    }
}
