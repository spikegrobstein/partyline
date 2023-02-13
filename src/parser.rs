use nom::{
    branch::alt, bytes::complete::tag, character::complete::alphanumeric1,
    character::complete::multispace1, character::complete::newline, combinator::opt, multi::many0,
    multi::separated_list0, IResult,
};

use crate::command::{Command, Modifier};

/// parse commands like on cli
/// prefix modifier of > or /
/// followed by first word
/// followed by (optoinally quoted) words

fn parse_modifier(input: &str) -> IResult<&str, Modifier> {
    let (input, m) = opt(alt((tag(">"), tag("/"))))(input)?;

    let modifier = match m {
        Some(">") => Modifier::Bracket,
        Some("/") => Modifier::Slash,
        Some(_) => unreachable!("can't reach."),
        None => Modifier::None,
    };

    Ok((input, modifier))
}

fn parse_cmd(input: &str) -> IResult<&str, String> {
    let (input, cmd) = alphanumeric1(input)?;
    let (input, _) = opt(multispace1)(input)?; // ignore next space

    Ok((input, cmd.to_owned()))
}

fn parse_args(input: &str) -> IResult<&str, Vec<String>> {
    let (input, mut args) = separated_list0(multispace1, alphanumeric1)(input)?;

    let args = args.iter_mut().map(|a| a.to_owned()).collect();

    Ok((input, args))
}

pub fn parse_command(input: &str) -> IResult<&str, Command> {
    let (input, modifier) = parse_modifier(input)?;
    dbg!(input);
    let (input, command) = parse_cmd(input)?;
    dbg!(input);
    let (input, args) = parse_args(input)?;
    dbg!(input);

    // ignore newlines
    let (input, _) = many0(newline)(input)?;

    Ok((
        input,
        Command {
            modifier,
            command,
            args,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_command_with_no_args() {
        let (input, command) = parse_command("echo").unwrap();

        assert_eq!(input, "");
        assert_eq!(command.command, "echo");
        assert!(command.args.len() == 0);
    }

    #[test]
    fn it_parses_normal_command() {
        let (input, command) = parse_command("echo this is really cool").unwrap();
        assert_eq!(input, "");

        assert_eq!(
            command,
            Command {
                modifier: Modifier::None,
                command: "echo".to_owned(),
                args: vec![
                    "this".to_owned(),
                    "is".to_owned(),
                    "really".to_owned(),
                    "cool".to_owned(),
                ]
            }
        )
    }

    #[test]
    fn it_parses_slash_command() {
        let (_input, command) = parse_command("/echo this is really cool").unwrap();

        assert_eq!(
            command,
            Command {
                modifier: Modifier::Slash,
                command: "echo".to_owned(),
                args: vec![
                    "this".to_owned(),
                    "is".to_owned(),
                    "really".to_owned(),
                    "cool".to_owned(),
                ]
            }
        )
    }

    #[test]
    fn it_parses_bracket_command() {
        let (_input, command) = parse_command(">echo this is really cool").unwrap();

        assert_eq!(
            command,
            Command {
                modifier: Modifier::Bracket,
                command: "echo".to_owned(),
                args: vec![
                    "this".to_owned(),
                    "is".to_owned(),
                    "really".to_owned(),
                    "cool".to_owned(),
                ]
            }
        )
    }
}
