fn main() {
    let farry = [32.0, 64.0, 94.0, 86.0];     
    // (32°F − 32) × 5/9
    let mut index = 0; 

    while index < 4 {
        // farry[index] = (farry[index] - 32) * (5/9);
        println!("{}",((farry[index] - 32.0) * (0.5556)));
        index += 1;
    };
}
