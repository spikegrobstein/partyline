use bytes::{Buf, BufMut, BytesMut};
use thiserror::Error;
use tokio_util::codec::{Decoder, Encoder};

use nom::character::is_newline;

use std::io;
use std::str;

use crate::command::Command;
use crate::parser;

#[derive(Error, Debug)]
pub enum CodecError {
    #[error("I/O Error")]
    IoError(#[from] io::Error),

    #[error("UTF8 error")]
    Utf8Error(#[from] str::Utf8Error),

    #[error("Parse error")]
    ParseError,

    #[error("Error reading a line...")]
    ReadLineError,
}

#[derive(Debug)]
pub enum Packet {
    Chat(String),
    Command(Command),
}

pub struct CmdCodec;

fn read_line(src: &mut BytesMut) -> Option<String> {
    let len = src.iter().position(|c| is_newline(*c))?;

    let slice = &src[..len];

    let s = str::from_utf8(slice).unwrap().to_owned();
    src.advance(len + 1);

    Some(s)
}

impl Decoder for CmdCodec {
    type Item = Packet;

    type Error = CodecError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // if there's nothing in the buffer, we got nothing.
        if src.is_empty() {
            return Ok(None);
        }

        let line = read_line(src).ok_or(CodecError::ReadLineError)?;

        if line.is_empty() {
            return Ok(None);
        }

        let packet = if line.starts_with('/') {
            // treat it as a command
            let (_, cmd) = parser::parse_command(&line).map_err(|_| CodecError::ParseError)?;

            Packet::Command(cmd)
        } else {
            Packet::Chat(line)
        };

        Ok(Some(packet))
    }
}

impl Encoder<String> for CmdCodec {
    type Error = CodecError;

    fn encode(&mut self, item: String, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.put_slice(item.as_bytes());
        dst.put_slice("\n".as_bytes());

        Ok(())
    }
}
