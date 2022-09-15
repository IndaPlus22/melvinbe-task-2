use std::io;
use std::io::prelude::*;

fn main() 
{
    let input = io::stdin();

    let lines = input
        .lock()
        .lines()
        .map(|_line| _line.ok().unwrap());

    // Solve problem line by line
    for line in lines
    {
        // Divide line into two i64 elements of a vector
        let numbers: Vec<i64> = line
        .split(" ") 
        .map(|x| x.parse().unwrap())
        .collect();

        // Calculate difference and absolute value before printing
        let mut output = numbers[0] - numbers[1]; 
        output = output.abs();
        println!("{}", output);
    }
}