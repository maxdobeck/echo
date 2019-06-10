use std::io;
use std::io::Write;

fn main() {
    let input = get_input();
    println!("{}", input);
}

fn get_input() -> String{
    print!("Enter your text: ");
    io::stdout().flush().unwrap();
    let mut val = String::new();

    io::stdin().read_line(&mut val)
        .expect("Error getting input");
    
    return val;
}

