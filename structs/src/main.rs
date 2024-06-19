#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
    active_count: u32,
}

impl User {
    fn create_new_user(&self) -> User {
        User {
            username: self.username.clone(),
            email: self.email.clone(),
            active: true,
            active_count: 1,
        }
    }

    fn get_email(&self) -> &str {
        &self.email
    }

    fn get_name(&self) -> String {
        self.username.clone()
    }
}

impl User {
    fn is_user_active(user: User) -> bool {
        if user.active {
            return true;
        }
        return false;
    }
}

fn main() {
    let user1 = User {
        username: String::from("name"),
        email: String::from("email"),
        active: true,
        active_count: 1,
    };

    let user2 = user1.create_new_user();

    println!("{:#?}", user2);
    println!("{}", user1.get_email());
    println!("{}", user1.get_name());

    let user_active = User::is_user_active(user1);
    println!("{:?}", user_active)
}
