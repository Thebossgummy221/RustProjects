use std::io;
use io::stdin;
use io::stdout;
use std::io::Write;

fn main() {
    let color1:i32;
    let color2:i32;
    let color3:i32;
    print!("Enter a red value (0-255): ");
    let mut s1 = String::new();
    stdout().flush().expect("");
    stdin().read_line(&mut s1).expect("");
    color1 = s1.trim().parse().expect("Input not an integer");
    print!("Enter a green value (0-255): ");
    let mut s1 = String::new();
    stdout().flush().expect("");
    stdin().read_line(&mut s1).expect("");
    color2 = s1.trim().parse().expect("Input not an integer");
    print!("Enter a blue value (0-255): ");
    let mut s1 = String::new();
    stdout().flush().expect("");
    stdin().read_line(&mut s1).expect("");
    color3 = s1.trim().parse().expect("Input not an integer");
    let invert_red = 255-color1;
    let invert_green = 255-color2;
    let invert_blue = 255-color3;
    println!("The inverted color is red={}, green={}, blue={}", invert_red, invert_green, invert_blue);
    let fp1:f32 = invert_red as f32 / 255.0;
    let fp2:f32 = invert_green as f32/255.0;
    let fp3:f32 = invert_blue as f32/255.0;
    println!("With floating points, that would be red={}, green={},\nblue={}", fp1, fp2, fp3);


}
