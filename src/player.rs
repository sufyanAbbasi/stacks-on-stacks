struct Player {
    life_total: u128,
    commander_damage: HashMap<Player, u32>,
    counters: HashMap<String, u32>,
    library: Vec<Card>,
    graveyard: Vec<Card>,
    exile: Vec<Card>,
    command_zone: Option<Card>,
    sideboard: Vec<Card>,
    hand_size: Option<u8>,
    hand: Vec<Card>,
}


// 2. Implement methods for the User struct
impl User {
    // An "associated function" (like a static method or constructor) to create a new instance
    fn new(email: String, username: String) -> Self {
        Self { // Self is an alias for the struct type (User)
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }

    // An instance method (takes &self, &mut self, or self) to access/modify instance data
    fn get_details(&self) -> String {
        format!("Username: {}, Email: {}, Active: {}", self.username, self.email, self.active)
    }

    // A method that modifies the instance (takes &mut self)
    fn deactivate(&mut self) {
        self.active = false;
    }
}

fn main() {
    // 3. Create an instance (instantiation)
    let mut user1 = User::new(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    // 4. Use the methods
    println!("{}", user1.get_details());

    user1.deactivate();

    println!("{}", user1.get_details());
}