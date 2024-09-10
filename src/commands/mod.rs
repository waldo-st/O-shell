pub mod cat;
pub mod cd;
pub mod cp;
pub mod echo;
pub mod ls;
pub mod mkdir;
pub mod mv;
pub mod pwd;
pub mod rm;

use crate::utils::error::ShellError;

// Trait commun pour toutes les commandes
pub trait Command {
    fn run(&self, args: &[String]) -> Result<(), ShellError>;
    fn name(&self) -> &'static str;
    // fn usage(&self) -> &'static str;
}

// Structure pour gérer toutes les commandes
pub struct Commands {
    commands: Vec<Box<dyn Command>>,
}

impl Commands {
    pub fn new() -> Self {
        let mut commands: Vec<Box<dyn Command>> = Vec::new();
        commands.push(Box::new(echo::Echo));
        commands.push(Box::new(cd::Cd));
        commands.push(Box::new(ls::Ls));
        // commands.push(Box::new(pwd::Pwd));
        // commands.push(Box::new(cat::Cat));
        // commands.push(Box::new(cp::Cp));
        // commands.push(Box::new(rm::Rm));
        // commands.push(Box::new(mv::Mv));
        // commands.push(Box::new(mkdir::Mkdir));
        
        Commands { commands }
    }

    pub fn get_command(&self, name: &str) -> Option<&Box<dyn Command>> {
        self.commands.iter().find(|cmd| cmd.name() == name)
    }

    // pub fn list_commands(&self) -> Vec<&'static str> {
    //     self.commands.iter().map(|cmd| cmd.name()).collect()
    // }
}