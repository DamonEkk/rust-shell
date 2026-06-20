#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage

    let command_directory = ["exit"];
    //let directory_size: usize = command_directory.len();

    
    loop{
        // Display $
        print!("$ ");
        io::stdout().flush().unwrap();
        // Get user command
        let command;
        command = get_user_input();
        let command = command.trim();
        let found_command: u8 = check_directory(&command, &command_directory); 
        if found_command == 1{
            break;
        }

        //println!("{}: command not found", command.trim());
        //testing
        println!("{command}: command not found")
    }
}

// Returns positional integer when command is found, if item is not found 0 will be returned.
fn check_directory(com: &&str, com_array: &[&str]) -> u8{
    let mut count = 0;
    for item in com_array{
        count += 1;
        if item.to_lowercase() == com.to_lowercase(){
            return count;    
        }     
    }

    return 0
}
 
// Get user input
fn get_user_input() -> String{
    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();
    command
}
