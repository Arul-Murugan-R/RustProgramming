// Check if a number is prime in Rust

use std::io;

fn main(){
    println!("Enter a number : ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Enter valid string");
    let mut val: i64 = input.trim().parse().unwrap();
    for i in 2..(val/2)+1 {
        if(val % i==0)
        {
            print!("Not a prime number");
            return;
        }
    }
    print!("Its a prime number");
    
}
