#[derive(Debug, PartialEq)]
pub enum Modifier {
    Slash,
    Bracket,
    None,
}

#[derive(Debug, PartialEq)]
pub struct Command {
    pub modifier: Modifier,
    pub command: String,
    pub args: Vec<String>,
}
