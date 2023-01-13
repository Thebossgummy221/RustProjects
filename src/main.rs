use std::io::*;
fn main() {
let grade: f32;
let  letter:String;
let mut response:String = String::new();
    print!("Enter the score of your exam: ");
    stdout().flush().expect("");
    stdin().read_line(& mut response).expect("Do not forget, do not forgive");
    grade = response.trim().parse().expect("Not an integer");
    if grade > 97 as f32{
        letter = "A+".to_string();
    }
    else if grade> 94 as f32 {
        letter = "A".to_string();
    }
    else if grade > 91 as f32{
    letter = "A-".to_string();
    }
    else if grade > 88 as f32 {
        letter = "B+".to_string();
    }
    else if grade > 85 as f32 {
        letter = "B".to_string();

    }
    else if grade > 82 as f32{
        letter = "B-".to_string();
    }
    else if  grade > 79 as f32 {
        letter = "C+".to_string();
    }
    else if grade > 76 as f32{
        letter = "C".to_string();
    }
    else if grade > 73 as f32{
        letter = "C-".to_string();
    }
    else if grade > 70 as f32{
        letter = "D+".to_string();
    }
    else if grade > 67 as f32{
        letter = "D".to_string();
    }
    else if grade > 64 as f32{
        letter = "D-".to_string();
    }
    else{
        letter= "F".to_string();
    }

    println!("Letter grade is: {}", letter);



}
