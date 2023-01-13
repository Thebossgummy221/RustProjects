use std::io::*;
fn main() {
    let mut day:String = String::new();
    print!("Enter the day: ");
    stdout().flush().expect("");
    stdin().read_line(& mut day).expect("Do not worry about this, plz");
    if day.trim().eq_ignore_ascii_case("monday"){
        println!("I have class today!");
    }
    else if day.trim().eq_ignore_ascii_case("wednesday"){
        println!("I have class today!");
    }
    else if day.trim().eq_ignore_ascii_case("friday") {
        println!("It's Friday! Friday! Gotta get down on Friday!");
    }
    else{
        println!("I should use this time to do my homework.");
    }

}
