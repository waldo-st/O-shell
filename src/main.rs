use shell::Shell;

pub mod commands;
pub mod utils;
mod shell;

fn main(){
    let mut shell = Shell::new();
    shell.run();
}