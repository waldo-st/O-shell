use std::io::{self, Write};
use crate::commands::{echo, Command};
use crate::utils::error::ShellError;

pub struct Shell{

}

impl Shell{
    pub fn new()->Self{
        Shell{

        }
    }

    pub fn run(&mut self){
        loop{
            print!("$");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            
            let input = input.trim();
            match self.excute_command(input) {
                Ok(_)=>{}
                Err(e) => eprintln!("Erreur : {:?}", e),
            }
        }

    }

    fn excute_command(&mut self, input: &str) -> Result<(), ShellError>{
        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts[0];
        let args = &parts[1..];
        let args_string = args.iter().map(|&s| s.to_string()).collect::<Vec<String>>();
        let args_string_slice:&[String]=&args_string;
        
        match command{
            "echo" => echo::Echo::run(&echo::Echo, args_string_slice),
            _=>Err(ShellError::UnknownCommand(command.to_string()))
        }
    }
}