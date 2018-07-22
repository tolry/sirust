extern crate lib;

use std::env;
use lib::Input;
use lib::StandardUnit;

fn main() {
    println!("{:?}", "22".parse::<u32>().unwrap());

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let a = Input {
        compound_unit: [(StandardUnit::Kilogram, 1)].iter().cloned().collect(),
        amount: -3.2
    };

    let b = Input {
        compound_unit: [(StandardUnit::Kilogram, 1)].iter().cloned().collect(),
        amount: 12.0
    };

    println!("{:?}", a.clone() + b.clone());
    println!("{:?}", a.clone() - b.clone());
    println!("{:?}", a.clone() * b.clone());
    //println!("{:?}", a.clone() / b.clone());
}
