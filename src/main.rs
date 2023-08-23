#![feature(test)]

pub mod mult;
pub mod search_2d;

use std::io;

fn mult_example() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let input = stdin.read_line(&mut buffer);

    match input {
        Ok(_) => match buffer.split_once(' ') {
            Some((a, b)) => {
                let au32 = a.parse::<u32>().unwrap();
                let bu32 = a.parse::<u32>().unwrap();
                println!(
                    "MY: {:?}; STD: {:?}",
                    mult::bin_mult(au32, bu32),
                    mult::std_mult(au32, bu32)
                );
            }
            None => panic!("Can't parse input"),
        },
        Err(err) => panic!("{:?}", err),
    }
}

fn search_2d_example() {
    let res = search_2d::search_matrix(vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]], 60);
    println!("{}", res);
}

fn main() {
    search_2d_example()
}
