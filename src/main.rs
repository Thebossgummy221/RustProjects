use std::io::*;

fn main() {
let quarter_num : i32;
let dime_num : i32;
let nickel_num : i32;
let pennies_num : i32;
print!("Enter the number of quarters: ");
let mut response = String::new();
stdout().flush().expect("This is fine");
stdin().read_line(& mut response).expect("Inputs are bad");
quarter_num = response.trim().parse().expect("Not an integer");
print!("Enter the number of dimes: ");
    let mut response = String::new();
    stdout().flush().expect("This is fine");
    stdin().read_line(& mut response).expect("Inputs are bad");
dime_num = response.trim().parse().expect("Not an integer");
print!("Enter the number of nickels: ");
    let mut response = String::new();
    stdout().flush().expect("This is fine");
    stdin().read_line(& mut response).expect("Inputs are bad");
    nickel_num = response.trim().parse().expect("Not an integer");
print!("Enter the number of pennies: ");
    let mut response = String::new();
    stdout().flush().expect("This is fine");
    stdin().read_line(& mut response).expect("Inputs are bad");
    pennies_num= response.trim().parse().expect("Not an integer");
    println!();
    println!("You entered {} quarters. ", quarter_num);
    println!("You entered {} dimes. ", dime_num);
    println!("You entered {} nickels.", nickel_num);
    println!("You entered {} pennies.", pennies_num);
    println!();
    let total:i32 = 25*quarter_num + 10*dime_num + 5*nickel_num + pennies_num;
    let dollar: i32 = total/100;
    let cents: i32 = total % 100;
    println!("Your total is {} dollars and {} cents.", dollar, cents);

}
