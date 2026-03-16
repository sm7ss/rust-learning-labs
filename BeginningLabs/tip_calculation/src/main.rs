use std::io;
use std::cmp::Ordering;

const MAX_TIP: f64 = 100.00;
const MIN_TIP: f64 = 1.00;

fn main() {
    println!("-- Calculate Your Tip --");
    
    println!("what is the total of your order?");
    let total= loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read your total order");
        
        match input.trim().parse::<f64>() {
            Ok(num) => break num, 
            Err(_) => {
                println!("Your total order should be a float");
                continue;
            }
        }
    };
    
    println!("How much percentage would you like to leave as a tip?");
    let percentage = loop {
        let mut input_per= String::new();
        
        io::stdin()
        .read_line(&mut input_per)
        .expect("Failed to read your percent tip");
        
        match input_per.trim().parse::<f64>() {
            Ok(num) => break num, 
            Err(_) => {
                println!("Your percent tip should be a float");
                continue;
            }
        }
    };
    
    
    
    
    
    
    
    
}
