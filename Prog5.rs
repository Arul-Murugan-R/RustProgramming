// Given a sorted array of integers, implement a function that returns the median of the array.

use std::io;

fn main() {
    println!("Enter the numbers : ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut arr: Vec<usize> = input.trim().split_whitespace()
        .map(|x| x.parse().unwrap()).collect();
    let len = arr.len();
    let k = len/2;
    let mut backup = 0;
    for i in 0..arr.len() - 1{
        let mut small = i;
        for j in i+1..arr.len(){
            if arr[j] < arr[small]{
                small = j;
            }
        }
        if len % 2 == 0 && k - 1 == i{
            backup = arr[small];
        }
        if k == i {
            let mut median = arr[small];
            if len % 2 == 0{
                median = median + backup;
            }
            println!("the median value in the given array is {}",median);
            return;
        }
        let temp = arr[i];
        arr[i] = arr[small];
        arr[small] = temp;
    }
}