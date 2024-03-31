// Given a string of words, implement a function that returns the shortest word in the string.

use std::io;
fn main(){
    println!("Enter a string : ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Enter valid string");
    let mut small = input.len();
    let mut splitWords: Vec<&str> = input[0..small - 1].split(' ').collect();
    // println!("{:?}",splitWords);
    let mut smallIndex = 0;
    let mut i = 0;
    for str in splitWords.iter(){
        let mut len = str.len();
        if len < small {
            small = len;
            smallIndex = i;
        }
        i = i + 1;
    }
    println!("The smallest word in the gn string : {} ",splitWords[smallIndex])
}
