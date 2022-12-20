use std::{string, vec, os::unix::prelude::PermissionsExt};
use rand::{thread_rng, Rng};

use std::io;

fn main() {
    let alphabet:Vec<&str> = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
    let numbers: Vec<&str> = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"];
    let special: Vec<&str> = vec!["_", "-", "!", "@", "!", "?", "{", ")", "*", "&"];


    let mut password_letters: Vec<&str> = Vec::new();
    let mut password_numbers: Vec<&str> = Vec::new();
    let mut password_special: Vec<&str> = Vec::new();
    let mut weirdpass: Vec<&str> = Vec::new();
    let mut pass: Vec<&str> = Vec::new();

    println!("Welcome to the password generater!");
    println!("1. how many charaters do you want");
    
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("this is not allowed charater");
    let s1 = input1.trim();
    let int1:i32 = s1.parse().unwrap();
    let uint1: u32 = int1.clone() as u32;

    for number in 0..int1 {
        let mut rng = thread_rng();
        let n: usize = rng.gen_range(0..26);
        let letter: &str = alphabet[n];
        password_letters.push(letter);
    }

    println!("2. how many numbers do you want");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("this is not allowed charater");
    let s2 = input2.trim();
    let int2:i32 = s2.parse().unwrap();
    let uint2: u32 = int2.clone() as u32;

    for number in 0..int2 {
        let mut rng = thread_rng();
        let n: usize = rng.gen_range(0..9);
        let number: &str = numbers[n];
        password_numbers.push(number);
    }
    println!(" how many special charaters do you want");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("this is not allowed charater");
    let s3 = input3.trim();
    let int3:i32 = s3.parse().unwrap();
    let uint3: u32 = int3.clone() as u32;

    for number in 0..int3 {
        let mut rng = thread_rng();
        let n: usize = rng.gen_range(0..9);
        let letter: &str = special[n];
        password_special.push(letter);
    }
    let int4: i32 = (int1 + int2 + int3) / 3;

    let mut x: usize = 0;
    for number in 0..int4 {
        let value_1: &str = password_letters[x];
        let value_2: &str = password_numbers[x];
        let value_3: &str = password_special[x];
        weirdpass.push(value_1);
        weirdpass.push(value_2);
        weirdpass.push(value_3);
        x = x + 1;
    }
    let int5 = int1 + int2 + int3;
    let uint4: u32 = uint1 + uint2 + uint3;
    let usizeint: usize = uint4.clone() as usize;
    
    for number in 0..int5 {
        let mut rng = thread_rng();
        let range = rng.gen_range(0..usizeint);
        let value = weirdpass[range];
        pass.push(value);
    }
    let finalpass: String = pass.into_iter().collect();
    print!("{}", finalpass);

}
    
 