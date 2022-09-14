use std::io;
use std::io::prelude::*;

fn main() 
{
    let input = io::stdin();

    let mut lines = input
        .lock()
        .lines()
        .map(|_line| _line.ok().unwrap());

    let mut n: usize = lines.next()
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    if n % 2 == 1
    {
        n += 1;
    } 

    let mut numbers: Vec<u32> = lines.next()
        .unwrap()
        .trim()
        .split(" ")
        .map(|x| x.parse().unwrap())
        .collect();

    numbers.sort();
    numbers.reverse();

    let mut output:u32 = 0;
    for i in 0..(n / 2)
    {
        output += numbers[i];
    }

    println!("{}", output);
}