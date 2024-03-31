// Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.

use std::io;

fn main() {
    println!("Enter the numbers : ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<usize> = input.trim().split_whitespace()
        .map(|x| x.parse().unwrap()).collect();
    println!("Enter the number to search : ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut val: i64 = input.trim().parse().unwrap();
    let mut index = 0;
    for i in arr.iter(){
        if i == &(val as usize){
            println!("The number {} is found in index {} ",val,index);
            return;
        }
        index = index + 1;
    }
    println!("-1");
}