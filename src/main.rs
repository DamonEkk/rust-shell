#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage

    let command_directory = ["exit", "echo", "type"];
    let executables = [];
    //let directory_size: usize = command_directory.len();

    
    loop{
        // Display $
        print!("$ ");
        io::stdout().flush().unwrap();

        // Get user command
        let command;
        command = get_user_input();
        let command = command.trim();
        let args: Vec<&str> = command.split_whitespace().collect();
        let found_command: u8 = check_directory(&args[0], &command_directory); 

        // Execute commands
        if found_command == 1{
            break;
        }
        else if found_command == 2{
            echo(&args);         
        }
        else if found_command == 3{
            type_command(&command_directory, &executables, &args);
        }
        else{
            println!("{command}: command not found")
        }
    }
}

fn type_command(com_array: &[&str], executables_array: &[&str], args: &Vec<&str>){
    // No builtins yet, add

    if args.len() > 2{
        println!("more than one arg");
    }

    let type_text = args[1];

    for item in com_array{
        if item.to_lowercase() == type_text.to_lowercase(){
            println!("{type_text} is a shell builtin");
            return;
        }
    }
    
    println!("{type_text}: not found");
    return

}

// Handles echo logic.
fn echo(args: &Vec<&str>){
    let mut count = 0;
    for item in args{
        if count != 0{
            if count > 1{
                print!(" ");
            }
            print!("{item}");
        }
        count += 1; 
    }
    println!(""); 
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
