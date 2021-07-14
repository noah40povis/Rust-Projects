fn main() {
    let s1 = String::from("hello");
    // The & are references, and they allow you to refer to some value without 
    // taking ownership of it. 
    // So in this line below calculate_length does not own s1 and therefore 
    // the value it points to will not be dropped when the reference goes out of scope. 
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
 // it refers to, nothing happens. 

 // Also it is important to note that references are immutable 

//  mutable references have one big restriction: 
//  you can have only one mutable reference to a particular piece of data in a particular scope. 


