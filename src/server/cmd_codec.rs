use tokio_util::codec::{Decoder, Encoder};
use bytes::{BufMut, BytesMut};
use thiserror::Error;

use std::str;
use std::io;

use crate::parser;
use crate::command::Command;

#[derive(Error, Debug)]
pub enum CodecError {
    #[error("I/O Error")]
    IoError(#[from] io::Error),

    #[error("Parse error")]
    ParseError(#[from] str::Utf8Error)
}

pub struct CmdCodec;

impl Decoder for CmdCodec {
    type Item = Command;

    type Error = CodecError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let line = str::from_utf8(&src[..]).map_err(CodecError::ParseError)?;
        let (_input, command) = parser::parse_command(line).unwrap();

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

