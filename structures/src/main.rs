struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool, 
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    }; 
    
    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    }; 

    println!("user1: {}", user1.email);
    println!("user2: {}", user2.email);
    println!("user3: {}", user3.email);

    let x = String::from("Boy");
    let y = String::from("toy");

    let z = build_user(x, y); 

    println!("{}",z.email)
}

fn build_user(email: String, username: String) -> User {
    User {
        email,  // short hand for - email: email 
        username, // short hand for - username: username
        active: true,
        sign_in_count: 1,
    } 
}
