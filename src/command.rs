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

impl Command {
    pub fn to_string(&self) -> String {
        format!("{:#?}", self)
    }
}
