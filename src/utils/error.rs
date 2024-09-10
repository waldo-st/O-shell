#[derive(Debug)]
pub enum ShellError {
    UnknownCommand(String),
    // Ajoutez d'autres types d'erreurs selon vos besoins
}