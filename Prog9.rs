use std::io;
fn main(){
    println!("Enter the string to reverse : ");
    let mut str = String::new();
    io::stdin().read_line(&mut str).expect("Enter valid string");
    let mut rev =String::new();
    let mut i = str.len() - 1;
    while i > 0 {
        rev.push_str(&str[i-1 .. i]);
        i = i - 1;
    }
    println!("The reverse of given string : {}",rev);
    
}
