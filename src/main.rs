use std::io::{stdin, stdout, Write};

fn main() {
    get_input_message();
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
