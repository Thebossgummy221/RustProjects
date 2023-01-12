use std::io;
use std::io::{stdout, Write};
use io::*;


fn main() {
    let am_owed: i32;
    let apr: f32;
    print!("Amount owed: $");
    let mut response:String = String::new();
    stdout().flush().expect("");
    stdin().read_line(& mut response).expect("");
    am_owed = response.trim().parse().expect("Not an integer");
    print!("APR: ");
    let mut response:String = String::new();
    stdout().flush().expect("");
    stdin().read_line(& mut response).expect("");
    apr = response.trim().parse().expect("Not a float");
    let month_percentage_rate = apr/ 12.0;
    println!("Monthly percentage rate: {}",month_percentage_rate );
    let min_payment =  am_owed as f32 * apr /12.0 / 100.0;
    print!("Minimum payment: {}", min_payment);



}
