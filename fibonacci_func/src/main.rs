use std::io;

fn main() {
        println!("Please input number:");

        let mut n = String::new();

        io::stdin()
            .read_line(&mut n)
            .expect("failed to read line");
    
        let n: u32 = n.trim().parse().expect("Please type a number");
        let mut a = 0;
        let mut b = 1;

        if n<a {
            println!("Incorrect Input");
        } else if n==a {
            println!("N equals: {}", 0)
        } else if n==b {
            println!("N equals: {}", 1)
        } else {
            for _ in 2..n+1 {
                let c = a + b;
                a = b;
                b = c;
            println!("Solution: {}", b) 
            }
            }
}   
