#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;
use is_executable::IsExecutable;
use std::fs;

fn main() {
    // TODO: Uncomment the code below to pass the first stage

    let command_directory = ["exit",
        "echo",
        "type"];
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

        match found_command {
            1 => break,
            2 => echo(&args),
            3 => type_command(&command_directory, &executables, &args[1]),
            _ => println!("{command}: command not found")
        }

    }
}

fn check_executable(type_text: &str, path: OsString) -> bool {
    let invalid_path = String::from("invalid");
    let path_string = match path.into_string(){
        Ok(valid) => valid,
        Err(invalid_path) => String::new(),
    };
    
    for item in env::split_paths(&path_string){

        let sub_path = Path::new(&item);
        let p_str = sub_path.join(type_text);
        //let p_str = format!("{}/{}", item, type_text);

        println!("{:?}", p_str);
        if command_in_path(&p_str){  
            let command = Path::new(&p_str);
            if command.is_executable(){
                println!("{} is {}", item.to_string_lossy(), command.to_string_lossy());
                return true;
            }
        }
    }
    false
}

fn command_in_path(p_str: &PathBuf) -> bool{
    if fs::metadata(p_str).is_ok(){
        return true;
    }
    false
}

// checks the type of command 
fn type_command(com_array: &[&str], executables_array: &[&str], type_text: &str){
    let error = String::from("invalid");
    // Builtin commands
    for item in com_array{
        if item.to_lowercase() == type_text.to_lowercase(){
            println!("{type_text} is a shell builtin");
            return;
        }
    }
    
    // Search executables
    let found = match std::env::var_os("PATH"){
        Some(executables) => check_executable(&type_text, executables),
        None => false,
    };
    if found {return}
    
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
