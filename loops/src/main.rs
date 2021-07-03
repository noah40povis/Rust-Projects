fn main() {
    let a = [10,20,30,40,50];
    let mut index =0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

// a more concise and faster version 

// fn better_main() {
//     let a = [10,20,30,40,50];

//     for element in a.iter() {
//         println!("the value is: {}", element);
//     }
// }