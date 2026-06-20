#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage

    
    loop{
        // Display $
        print!("$ ");
        io::stdout().flush().unwrap();
        // Get user command
        let mut command = String::new();
        command = get_user_input();
        println!("{}: command not found", command.trim()); 
    }


}

fn get_user_input() -> String{
    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();
    command
}
