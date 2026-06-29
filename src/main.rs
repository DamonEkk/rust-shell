#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::ffi::OsString;
use std::path::Path;
use is_executable::IsExecutable;
use std::process::Command;
use std::os::unix::process::CommandExt;

mod builtins;

fn main() {
    //let args: Vec<String> = env::args().collect();

    let command_directory = ["exit",
        "echo",
        "type",
        "pwd"];
    
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
            2 => builtins::echo(&commands_split),
            3 => builtins::type_command(&command_directory, &commands_split[1]),
            _ => execute_command(&command, &commands_split),
        }

    }
}

fn execute_command(command :&str, args :&Vec<&str>){
    let no_arg_command_list: Vec<&str> = command.split_whitespace().collect();
    let no_arg_command = no_arg_command_list[0];
    let program_name = args[0];
    let no_print = true;

    let found = match std::env::var_os("PATH"){
        Some(executables) => builtins::check_executable(&no_arg_command, executables, no_print),
        None => false,
    };
    if found {
        let executable = match std::env::var_os("PATH"){
            Some(executable) => get_executable(&no_arg_command, executable),
            None => String::new(),
        };

        let mut child = Command::new(&executable)
            .arg0(program_name)
            .args(&args[1..])
            //.args(args)
            .spawn()
            .expect("failed to execute");

        child.wait().expect("failed to wait");
        return

    }
    println!("{command}: command not found")
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

        if builtins::command_in_path(&p_str){  
            let command = Path::new(&p_str);
            if command.is_executable(){
                let command_executable = command.to_string_lossy().to_string();
                return command_executable;
            }
        }
    }
    return invalid_path
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
