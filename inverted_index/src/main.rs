pub fn create_greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    println!("What's your name?");
    
    let mut input = String::new(); // creates an empty string to store input

    let stdin = std::io::stdin(); // standar input handler

    let result = stdin.read_line(&mut input);

    match result {
        Ok(_number_of_bytes_read) => { // _number_of_bytes_read because when read_line it returns the  number of bytes that were read
            // _ in fron of a var is rust is we won't use it
            println!("{}", create_greeting(input.trim()));
        }
        Err(error) => { // => is used to separate the pattern from the code in match
            println!("Failed to read line: {}", error)
        }
    }
    // Efficient way: 
    //std::io::stdin()
    //     .read_line(&mut input)
    //     .expect("Failed to read line");
}
