// cargo run --example 09_command_bus

// Command Bus with more than one command

use std::any::{Any, TypeId};
use std::collections::HashMap;

// Traits
pub trait Command {
    type Output; // kind of placeholder for a type to be determined later (String, bool...)
}

pub trait Handler<C: Command> {
    fn handle(&self, cmd: C) -> C::Output;
}

// Commands
struct CreateUser {
    pub name: String,
}

impl Command for CreateUser {
    type Output = String;
}

struct DeleteUser {
    pub id: u32,
}

impl Command for DeleteUser {
    type Output = bool;
}

// Handlers
struct CreateUserHandler;

impl Handler<CreateUser> for CreateUserHandler {
    fn handle(&self, cmd: CreateUser) -> String {
        format!("Utilisateur créé: {}", cmd.name)
    }
}

struct DeleteUserHandler;

impl Handler<DeleteUser> for DeleteUserHandler {
    fn handle(&self, cmd: DeleteUser) -> bool {
        println!("Utilisateur {} supprimé", cmd.id);
        true
    }
}

// CommandBus
struct CommandBus {
    handlers: HashMap<TypeId, Box<dyn Any>>,
}

impl CommandBus {
    pub fn new() -> Self {
        CommandBus { handlers: HashMap::new() }
    }

    pub fn register<C, H>(&mut self, handler: H)
    where
        C: Command + 'static,
        H: Handler<C> + 'static,
    {
        self.handlers.insert(TypeId::of::<C>(), Box::new(handler));
    }

    pub fn dispatch<C, H>(&self, cmd: C) -> C::Output
    where
        C: Command + 'static,
        H: Handler<C> + 'static,
    {
        let type_id = TypeId::of::<C>();
        let handler = self.handlers.get(&type_id).unwrap_or_else(|| panic!("Aucun handler enregistré pour la commande {type_id:?}"));

        let handler = handler.downcast_ref::<H>().expect("Mauvais type de handler");

        handler.handle(cmd)
    }
}

fn main() {
    let mut bus = CommandBus::new();

    bus.register::<CreateUser, CreateUserHandler>(CreateUserHandler);
    bus.register::<DeleteUser, DeleteUserHandler>(DeleteUserHandler);

    let creation_result = bus.dispatch::<CreateUser, CreateUserHandler>(CreateUser { name: "Alice".into() });
    println!("{creation_result}");

    let deletion_result = bus.dispatch::<DeleteUser, DeleteUserHandler>(DeleteUser { id: 42 });
    println!("Suppression réussie ? {deletion_result}");

    use_bus(&bus);
}

fn use_bus(bus: &CommandBus) {
    let result = bus.dispatch::<CreateUser, CreateUserHandler>(CreateUser { name: "Bob".into() });
    println!("Dans une autre fonction: {result}");
}
