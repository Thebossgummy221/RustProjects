use std::io;
use io::stdin;
use std::io::{stdout, Write};

fn main() {


    let mut name = String::new();
    let mut a_name = String::new();
    let mut verb = String::new();
    let mut adverb = String::new();

    print!("Enter a name: ");
    stdout().flush().expect("TODO: panic message");
    stdin().read_line(&mut name).expect("");
    print!("Enter another name: ");
    stdout().flush().expect("TODO: panic message");
    stdin().read_line(&mut a_name).expect("");
    print!("Enter a verb: ");
    stdout().flush().expect("TODO: panic message");
    stdin().read_line(& mut verb).expect("");
    print!("Enter an adverb: ");
    stdout().flush().expect("TODO: panic message");
    stdin().read_line(& mut adverb).expect("");
    print!("Once upon a time, there was a person named {} who had a child named " , name);
 //   println!("{}. This child would {} {} while singing to strangers. ", a_name, verb, adverb);

}
