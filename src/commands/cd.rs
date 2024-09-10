use super::Command;
pub struct Cd;

impl Command for Cd{
    fn run(&self, args: &[String]) -> Result<(), crate::utils::error::ShellError> {
        Ok(())
    }
    fn name(&self) -> &'static str {
        "echo"
    }
}