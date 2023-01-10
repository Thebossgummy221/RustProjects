use std::io;
use io::*;

fn main()
{
    let day: i32;
    let n_month: i32;
    let mut month_as_words:String = String::new();
    let year:i32;
    print!("Enter the day (number): ");
    let mut response:String = String::new();
    stdout().flush().expect("Should advance and allow input to be entered as same line as previous statement");
    stdin().read_line(&mut response).expect("Will parse");
    day = response.trim().parse().expect("Not an integer");
    print!("Enter the month (number): ");
    let mut response:String = String::new();
    stdout().flush().expect("Something occurred here, why");
    stdin().read_line(& mut response).expect("Will parse");
    n_month= response.trim().parse().expect("Not a number");
    print!("Enter the month (String): ");
    stdout().flush().expect("");
    stdin().read_line(&mut month_as_words).expect("");
    print!("Enter the year (number): ");
    let mut response:String = String::new();
    stdout().flush().expect(" ");
    stdin().read_line(& mut response).expect("");
    year = response.trim().parse().expect("Not an integer");
    println!("Here are some ways to represent the date:");
    println!("{}/{}/{}", n_month, day, year);
    println!("{}/{}/{}", year, day, n_month);
    println!("{}/{}/{}", day, n_month, year);
    println!("{} {}, {}", month_as_words.trim(), day, year);
    println!("{} {} {}", day, month_as_words.trim(), year);





}
