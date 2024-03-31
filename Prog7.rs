// Implement a function that returns the kth smallest element in a given array.

use std::io;

fn main() {
    println!("Enter the numbers : ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut arr: Vec<usize> = input.trim().split_whitespace()
        .map(|x| x.parse().unwrap()).collect();
    println!("Enter the kth term number : ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let k: i64 = input.trim().parse().unwrap();
    // let mut index = 0;
    for i in 0..arr.len() - 1{
        let mut small = i;
        for j in i+1..arr.len(){
            if arr[j] < arr[small]{
                small = j;
            }
        }
        if k-1 == (i as i64){
            println!("the {}th smallest value in the given array is {}",k,arr[small]);
            return;
        }
        let temp = arr[i];
        arr[i] = arr[small];
        arr[small] = temp;
    }
    println!("-1");
}