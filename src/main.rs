#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;
use is_executable::IsExecutable;
use std::fs;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command_directory = ["exit",
        "echo",
        "type"];
    
    loop{
        // Display $
        print!("$ ");
        io::stdout().flush().unwrap();

        // Get user command
        let command = get_user_input();
        let command = command.trim();
        let commands_split: Vec<&str> = command.split_whitespace().collect();
        let found_command: u8 = check_directory(&commands_split[0], &command_directory); 

        match found_command {
            1 => break,
            2 => echo(&commands_split),
            3 => type_command(&command_directory, &commands_split[1]),
            _ => execute_command(&command, &args),
        }

    }
}

fn execute_command(command :&str, args :&Vec<String>){
    let no_arg_command_list: Vec<&str> = command.split_whitespace().collect();
    let no_arg_command = no_arg_command_list[0];
    
    let found = match std::env::var_os("PATH"){
        Some(executables) => check_executable(&no_arg_command, executables),
        None => false,
    };
    if found {
        let executable = match std::env::var_os("PATH"){
            Some(executable) => get_executable(&no_arg_command, executable),
            None => String::new(),
        };
        
        
        let mut child = Command::new(executable)
            .args(args)
            .spawn()
            .expect("failed to execute");

        child.wait().expect("failed to wait");

    }
    println!("{command}: command not found")
}


fn check_executable(type_text: &str, path: OsString) -> bool {
    let path_string = match path.into_string(){
        Ok(valid) => valid,
        Err(_invalid_path) => String::new(),
    };
    
    for item in env::split_paths(&path_string){

        let sub_path = Path::new(&item);
        let p_str = sub_path.join(type_text);

        if command_in_path(&p_str){  
            let command = Path::new(&p_str);
            if command.is_executable(){
                println!("{} is {}", type_text, command.to_string_lossy());
                return true;
            }
        }
    }
    false
}

fn get_executable(command_string: &str ,path: OsString) -> String{
    let invalid_path = String::from("invalid");
    let path_string = match path.into_string(){
        Ok(valid) => valid,
        Err(_invalid_path) => String::new(),
    };
    
    for item in env::split_paths(&path_string){

        let sub_path = Path::new(&item);
        let p_str = sub_path.join(command_string);

        if command_in_path(&p_str){  
            let command = Path::new(&p_str);
            if command.is_executable(){
                let command_executable = command.to_string_lossy().to_string();
                return command_executable;
            }
        }
    }
    return invalid_path
}

fn command_in_path(p_str: &PathBuf) -> bool{
    if fs::metadata(p_str).is_ok(){
        return true;
    }
    false
}

// checks the type of command 
fn type_command(com_array: &[&str], type_text: &str){
    if com_array.len() < 2{
        println!("{type_text}: not found");
        return
    }

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
