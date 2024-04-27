use std::backtrace;

#[derive(Clone, Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    fn print_username(&self) {
        println!("username: {}", self.username);
    }

    // Field shadowing
    fn active(&self) -> bool {
        self.active
    }

    // Executing this method will put the current object from out of scope
    fn reset_count(mut self) -> User {
        self.sign_in_count = 0;
        self
    }
}

impl User {
    fn is_same_user(&self, next: &User) -> bool {
        self.username == next.username && self.email == next.email
    }
    fn create_test(email: String) -> Self {
        Self {
            active: true,
            username: String::from("test"),
            email: email, 
            sign_in_count: 0
        }
    }
}

// Touple Strcts
#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

// Unit like Struct
#[derive(Debug)]
struct AlwaysEqual;

fn main() {
    // Creating with all the values
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("\nuser1: {:#?}", user1);

    // Cloning a value from old object and replaceing it.
    let mut user2 = user1.clone();
    user2.email = String::from("yashsinghbishen@gmail.com");
    println!("\nuser2: {:#?}", user2);

    // Creating a new with method
    let mut user3 = build_user(String::from("test@mail.com"), String::from("test123"));
    user3.email = String::from("yashsinghbishen@gmail.com");
    println!("\nuser3: {:#?}", user3);

    // Creating new object with few details and from other structs with cloning
    let user4 = User {
        active: false,
        ..user3.clone()
    };
    println!("\nuser4: {:#?}", user4);

    println!(
        "\nAll objects\n{:#?}\n{:#?}\n{:#?}\n{:#?}\n",
        user1, user2, user3, user4
    );

    // Creating new object with few details and from other structs with transferring details
    let user5 = User {
        sign_in_count: 0,
        ..user3 // After this point user3 won't have email and user name as it's values has been borrowed by user5
    };

    println!("\nuser5: {:#?}", user5);
    // println!("\nuser3: {:#?}", user3); //This won't print

    let user6 = User {
        email: String::from("email@test.com"),
        username: String::from("test"),
        ..user5 // user5 will still work as the scaler type values are copied(ie. bool and u65)
    };
    println!("\nuser6: {:#?}", user6);
    println!("\nuser5: {:#?}", user5);

    let user7 = build_user_short_hand(String::from("test@mail.com"), String::from("test123"));
    println!("\nuser7: {:#?}", user7);

    user7.print_username();
    // Using method
    if !user7.active() {
        println!("\nuser is inactive.")
    }
    // Using property
    println!("User's active status is: {}", user7.active);

    if user6.is_same_user(&user7) {
        println!("User6 and user7 are same");
    } else {
        println!("User6 and user7 are not same");
    }

    let new_user7 = user7.reset_count();
    println!("\nnew_user7: {:#?}", new_user7);
    // println!("\nuser7: {:#?}", user7); // This will throw error as after calling user7.reset_count(), user7 is out of scope

    // Associated functions
    let user8 = User::create_test(String::from("test@mail.com"));
    println!("\nuser8: {:#?}", user8);


    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black: {:#?}", black);
    println!("origin: {:#?}", origin);

    let subject = AlwaysEqual;
    println!("subject: {:#?}", subject);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn build_user_short_hand(email: String, username: String) -> User {
    User {
        active: false,
        username,
        email,
        sign_in_count: 10,
    }
}
