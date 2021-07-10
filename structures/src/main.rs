struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool, 
}

let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

fn main() {
    println!("Hello, world!");
}

user1.email = String::from("anotheremail@example.com");


fn build_user(email: String, username: String) -> User {
    User {
        email,  // short hand for - email: email 
        username, // short hand for - username: username
        active: true,
        sign_in_count: 1,
    }
}


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