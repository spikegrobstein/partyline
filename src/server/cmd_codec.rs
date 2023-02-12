use tokio_util::codec::{Decoder, Encoder};
use bytes::{Buf, BufMut, BytesMut};
use thiserror::Error;

use nom::character::{is_newline, is_space};

use std::str;
use std::io;

use crate::parser;
use crate::command::Command;

#[derive(Error, Debug)]
pub enum CodecError {
    #[error("I/O Error")]
    IoError(#[from] io::Error),

    #[error("UTF8 error")]
    Utf8Error(#[from] str::Utf8Error),

    #[error("Parse error")]
    ParseError,
}

pub struct CmdCodec;

impl Decoder for CmdCodec {
    type Item = Command;

    type Error = CodecError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // ignore newlines and whitespace
        while ! src.is_empty() && (is_space(src[0]) || is_newline(src[0])) {
            src.advance(1);
        }

        // if there's nothing in the buffer, we got nothing.
        if src.is_empty() {
            return Ok(None);
        }

        // try to parse the line.
        let len = src.len();
        let line = str::from_utf8(&src[..])?;
        let (remaining, command) = parser::parse_command(line).map_err(|_| CodecError::ParseError)?;

        let advance_count = len - remaining.len();
        dbg!("Advancing", advance_count);
        src.advance(advance_count);

        Ok(Some(command))
    }
}

impl Encoder<&str> for CmdCodec {
    type Error = CodecError;

    fn encode(&mut self, item: &str, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.put_slice(item.as_bytes());
        dst.put_slice("\n".as_bytes());

        Ok(())
    }
}

