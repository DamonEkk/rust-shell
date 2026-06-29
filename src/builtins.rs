#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;
use is_executable::IsExecutable;
use std::fs;
 
pub fn type_command(com_array: &[&str], type_text: &str){
    let no_print = false;

    if com_array.len() < 2{
        println!("{type_text}: not found");
        return
    }

    let _error = String::from("invalid");
    // Builtin commands
    for item in com_array{
        if item.to_lowercase() == type_text.to_lowercase(){
            println!("{type_text} is a shell builtin");
            return;
        }
    }
    
    // Search executables
    let found = match std::env::var_os("PATH"){
        Some(executables) => check_executable(&type_text, executables, no_print),
        None => false,
    };
    if found {return}
    
    println!("{type_text}: not found");
    return
}

// Handles echo logic.
pub fn echo(args: &Vec<&str>){
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

pub fn pwd() -> std::io::Result<()> {
    let path = env::current_dir()?;
    println!("{}", path.display());
    Ok(())
} 


pub fn check_executable(type_text: &str, path: OsString, no_print: bool) -> bool {
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
                if !no_print{
                    println!("{} is {}", type_text, command.to_string_lossy());
                }
                return true;
            }
        }
    }
    false
}


pub fn command_in_path(p_str: &PathBuf) -> bool{
    if fs::metadata(p_str).is_ok(){
        return true;
    }
    false
}
