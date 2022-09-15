use std::io;
use std::io::prelude::*;

fn main() 
{
    let input = io::stdin();

    let mut lines = input
        .lock()
        .lines()
        .map(|_line| _line.ok().unwrap());

    // Get and parse n from first line
    let mut n: usize = lines.next()
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    // Add 1 if odd to make even
    if n % 2 == 1
    {
        n += 1;
    } 

    // Put numbers from second line into vector of u32
    let mut numbers: Vec<u32> = lines.next()
        .unwrap()
        .trim()
        .split(" ") // Seperate numbers
        .map(|x| x.parse().unwrap())
        .collect();

    numbers.sort(); // Sort from smallest to greatest
    numbers.reverse(); // -> Greatest to smallest

    let mut output:u32 = 0;
    // Loop through and add together first n / 2 numbers
    for i in 0..(n / 2)
    {
        output += numbers[i];
    }

    println!("{}", output);
}