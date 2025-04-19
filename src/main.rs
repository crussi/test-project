use std::io::{stdin, stdout, Write};

fn main() {
    get_input_message();
    get_input_number();
}


fn get_input_message() {
    let mut message = String::from("Enter a message: ");
    
    print!("{}", message);
    // Flush needed to ensure prompt appears before input
    stdout().flush().unwrap();

    message.clear(); // clear the original prompt
    stdin()
        .read_line(&mut message)
        .expect("Failed to read line");

    println!("Your message is: {}", message);
}

fn get_input_number() {
    let mut number = String::new();
    
    print!("Enter a number: ");
    stdout().flush().unwrap();

    stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let parsed: Result<i32, _> = number.trim().parse();

    match parsed {
        Ok(n) => println!("Your number is: {}", n),
        Err(_) => println!("⚠️ Warning: That wasn't a valid number."),
    }
}
