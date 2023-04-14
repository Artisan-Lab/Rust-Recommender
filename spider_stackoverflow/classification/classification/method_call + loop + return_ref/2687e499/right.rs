use std::collections::HashMap;

/// Represents remote user. Usually has fields,
/// but we omit them for the sake of example.
struct User;

impl User {
    /// Send data to remote user.
    fn send(&mut self, data: &str) {
        println!("Sending data to user: \"{}\"", data);
    }
}

/// A service that handles user data.
/// Usually has non-trivial internal state, but we omit it here.
struct UserHandler {
    users: HashMap<i32, User>,  // Maps user id to User objects.
    counter: i32,  // Represents internal state
}

impl UserHandler {
    fn handle_data(&mut self, user_id: i32, data: &str) {
        if let Some(user) = self.users.get_mut(&user_id) {
            Middle::new(&mut self.counter).handle_user_data(user, data);
        }
    }
}

struct Middle<'a> {
    counter: &'a mut i32,
}

impl<'a> Middle<'a> {
    fn new(counter: &'a mut i32) -> Self {
        Self {
            counter
        }
    }

    fn handle_user_data(&mut self, user: &mut User, data: &str) {
        user.send("Message received!");
        *self.counter += 1;
    }
}

fn main() {
    // Initialize UserHandler:
    let mut users = HashMap::new();
    users.insert(1, User{});
    let mut handler = UserHandler{users, counter: 0};
    
    // Pretend we got message from network:
    let user_id = 1;
    let user_message = "Hello, world!";
    handler.handle_data(user_id, &user_message);
}