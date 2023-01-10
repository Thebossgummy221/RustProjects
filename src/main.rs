use std::io::{stdin, stdout, Write};

fn main() {
   let mut item1 = String::new();
   let mut item2 = String::new();
   print!("What're you buying? ");
   stdout().flush().expect("");
   stdin().read_line (&mut item1).expect("Input is not a string");
   print!("How many? ");
   let mut value_response = String::new();
   stdout().flush().expect("");
   stdin().read_line(&mut value_response).expect("");
   let q1:i32 = value_response.trim().parse().expect("");
   print!("What do they cost? ");
   let mut value_response = String::new();
   stdout().flush().expect("");
   stdin().read_line(&mut value_response).expect("");
   let price1:f64 = value_response.trim().parse().expect("Input not an integer");
   println!();
   print!("What else're you buying? ");
   stdout().flush().expect("");
   stdin().read_line (&mut item2).expect("Input is not a string");
   print!("How many? ");
   let mut value_response = String::new();
   stdout().flush().expect("");
   stdin().read_line(&mut value_response).expect("");
   let q2:i32 = value_response.trim().parse().expect("");
   print!("What do they cost? ");
   let mut value_response = String::new();
   stdout().flush().expect("");
   stdin().read_line(&mut value_response).expect("");
   let price2:f64 = value_response.trim().parse().expect("");
   println!();
   println!("Your list:");
   println!("----");
   println!("{} ({})",item1.trim(),q1 );
   println!("${} (${} total)", price1, price1*q1 as f64);
   println!();
   println!("{} ({})",item2.trim(),q2 );
   println!("${} (${} total)", price2, price2*q2 as f64);
   println!();
   println!("Total Cost: {}", price1*q1 as f64 + price2*q2 as f64);
   println!("----");


}
