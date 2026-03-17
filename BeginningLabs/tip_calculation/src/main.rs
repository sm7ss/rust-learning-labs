use std::io;

const MAX_TIP: f64 = 100.00;
const MIN_TIP: f64 = 1.00;

fn main() {
    println!("-- Calculate Your Tip --");
    
    println!("what is the total of your order?");
    let total: f64 = loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read your total order");
        
        let input: f64 = match input.trim().parse::<f64>() {
            Ok(num) => num, 
            Err(_) => {
                println!("Your total order should be a float");
                continue;
            }
        };
        
        if input <= 0.0 {
            println!("Your total order should be greater than 0, cant be lower.");
            continue;
        } else {
            break input
        }
    };
    
    println!("How much percentage would you like to leave as a tip?");
    let percentage: f64 = loop {
        let mut input_per= String::new();
        
        io::stdin()
        .read_line(&mut input_per)
        .expect("Failed to read your percent tip");
        
        let input_per: f64 = match input_per.trim().parse::<f64>() {
            Ok(num) => num, 
            Err(_) => {
                println!("Your percent tip should be a float");
                continue;
            }
        };
        
        if input_per > MAX_TIP {
            println!("Your tip percent cannot be greater than 100%");
            continue;
        } else if input_per < MIN_TIP {
            println!("Yout tip percent cannot be less than 1%. You should leave at leat a 1% tip");
            continue;
        } else {
            break input_per
        }
    };
    
    let tip: f64 = total * (percentage / 100.0);
    let total_pay: f64= total + tip;
    
    println!(r#"
    ===========================
            Total Order 
    ===========================
     Total Order: ${:.2}
     Percent Tip: {:.2}%
     Total Tip: ${:.2} 
    ----------------------------
     Total Pay: ${:.2}
    "#, total, percentage, tip, total_pay);
}
