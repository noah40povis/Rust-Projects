fn main() {
    let s1 = String::from("hello john stewart welcome to saturday at the mirage");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a string
    (s, length)
}