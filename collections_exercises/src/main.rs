use std::io;
use std::collections::HashMap;
use std::num::ParseIntError;

fn median(arr: &mut [i32]) -> f32 {
    arr.sort();
    let len = arr.len();
    if len % 2 == 0 { 
        (arr[len / 2] + arr[len / 2 - 1]) as f32 / 2f32
    }
    else {
        arr[len / 2] as f32
    }
}

fn mode(arr: &[i32]) -> i32 {
    let mut hash = HashMap::new();
    
    arr.iter().copied().max_by_key(|&a| {
        let c = hash.entry(a).or_insert(0);
        *c += 1; *c
    }).expect("0 elements in arr")
}

fn average(arr: &[i32]) -> f32 {
    arr.iter().sum::<i32>() as f32 / arr.len() as f32
}

fn parse_input(str: &str) -> Result<Vec<i32>, ParseIntError> {
    let mut v: Vec<i32> = Vec::new();
    for s in str.split_whitespace() {
        v.push(s.parse()?);
    }
    Ok(v)
}

fn main() {
    // Given a list of integers, use a vector and return the median 
    // (when sorted, the value in the middle position) 
    // and mode (the value that occurs most often; a hash map will be helpful here) of the list.
    println!("Type in integers seperated by whitespaces: ");
    
    let mut input = String::new();
    let mut integers = 'outer: loop {
        io::stdin().read_line(&mut input).expect("Something went wrong");

        match parse_input(&input) {
            Ok(val) => break val,
            Err(e) => {
                println!("{e}.\nType new integers separated by whitespaces:");
                input.clear();
                continue 'outer;
            }
        }
    };

    println!("integers is -> {:#?}", &integers);
    println!("median is -> {:#?}", median(&mut integers));
    println!("mode is -> {:#?}", mode(&integers));
    println!("average is -> {:#?}", average(&integers));
}
