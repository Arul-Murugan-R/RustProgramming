// Implement a function that checks whether a given string is a palindrome or not.\

use std::io;

fn main(){
    println!("Enter the string : ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Enter valid string");
    let mut len = input.len();
    let mut i = 0;
    let mut last = len - 1;
    while i < last{
        if &input[i..i+1]!= &input[last - 1 ..last]{
            println!("The gn string is not a palindrome!!");
            return;
        }
        i = i + 1;
        last = last - 1;
    }
    println!("The gn string is a palindrome!!");
}
