use super::Command;
pub struct Ls;

impl Command for  Ls{
    fn run(&self, args: &[String]) -> Result<(), crate::utils::error::ShellError> {
        Ok(())
    }
    fn name(&self) -> &'static str {
        "Ls"
    }
}