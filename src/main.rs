extern crate core;

use std::io::*;
fn main() {
    let number:f32;
    let option: i32;
    print!("Welcome\nPlease input a number: ");
    let mut response:String = String::new();
    stdout().flush().expect("...");
    stdin().read_line(& mut response).expect("WHY!!!");
    number = response.trim().parse().expect("Not a float");
    println!();
    println!("What would you like to do to this number:");
    println!("0 - Get the additive inverse of the number");
    println!("1 - Get the reciprocal of the number");
    println!("2 - Square the number");
    println!("3 - Cube the number");
    println!("4 - Exit the program");
    println!();
    response.clear();
    stdin().read_line(& mut response).expect("");
    option = response.trim().parse().expect("Not an integer");
    println!();
    match option{
        0 => {
            println!("The additive inverse of {} is {}", number, number * -1.0 as f32 );
        }
        1 =>{
            println!("The reciprocal of {} is {} ", number, 1.0/number as f32);
        }
        2 => {
            println!("The square of {} is {}", number, number.powf(2.0));
        }
        3 =>{
            println!("The cube of {} is {}", number, number.powf(3.0));
        }
        4 =>{
        println!("Thank you, goodbye");
    }

        _=>{
        println!("Invalid input, please try again");
        }
    }





}
