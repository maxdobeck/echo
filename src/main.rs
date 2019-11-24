use std::io;
use std::io::Write;

fn main() {
    let input = get_input();
    let s: &str = &input[..];
    echo(s);
}

fn get_input() -> String {
    let prompt: String;
    prompt = "Enter your text:".to_string();
    print!("{}", prompt);

    io::stdout().flush().unwrap();
    let mut val = String::new();

    io::stdin()
        .read_line(&mut val)
        .expect("Error getting input");

    return val;
}

fn echo(input: &str) {
    for i in (1..input.len()).rev() {
        print!("{}\n", &input[..i]);
    }

}