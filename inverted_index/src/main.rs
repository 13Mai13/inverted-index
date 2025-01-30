pub fn create_greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    println!("What's your name?");
    
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    println!("{}", create_greeting(input.trim()));
}
