use super::Command;
pub struct Echo;

impl Command for Echo{
    fn run(&self, args: &[String]) -> Result<(), crate::utils::error::ShellError> {
        println!("{}",args.join(" "));
        Ok(())
    }
    fn name(&self) -> &'static str {
        "echo"
    }
}