/**
 * @file variables.rs
 * @description
 * @author Dushan Ranasinghage
 * @copyright Copyright 2024 - ResearchIt All Rights Reserved.
 */

fn main() {
    let num = 10;
    println!("Number: {}", num);

    // Mutable variable
    let mut name = "John";
    println!("My name is {} 1", name);
    name = "Doe";
    println!("My name is {} 2", name);

    // Constants
    const PI: f64 = 3.14;
    println!("PI Value: {}", PI);

    // Shadowing - similar to block scope in JS
    {
        let num = 20;
        println!("Number: {}", num);
    }

    println!("Number: {}", num);
}
