use nom::{
    bytes::complete::take, error::Error as NomErrorr, multi::length_data,
    number::complete::be_u32, Err as NomErr,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid magic bytes")]
    InvalidMagic,
    #[error("Unsupported version: {0}")]
    UnsupportedVersion(u8),
    #[error("Unknown type: {0}")]
    UnknownType(u8),
    #[error("Payload too long: {0}")]
    TooLong(usize),
    #[error("Nom error: {0}")]
    Nom(String),
}

/// Parse the entire packet from bytes
pub fn parse_packet(input: &[u8]) -> Result<String, ParseError> {
    let (input, magic) =
        take(2usize)(input).map_err(|e: NomErr<NomErrorr<_>>| ParseError::Nom(e.to_string()))?;

    if magic != [0xCA, 0xFE] {
        return Err(ParseError::InvalidMagic);
    }

    let (input, version_slice) =
        take(1usize)(input).map_err(|e: NomErr<NomErrorr<_>>| ParseError::Nom(e.to_string()))?;
    let version = version_slice[0];
    if version != 1 {
        return Err(ParseError::UnsupportedVersion(version));
    }

    let (input, typ_slice) =
        take(1usize)(input).map_err(|e: NomErr<NomErrorr<_>>| ParseError::Nom(e.to_string()))?;
    let typ = typ_slice[0];
    if typ > 1 {
        panic!("Critical: Unknown packet type {}", typ);
    }

    let (_, payload) =
        length_data(be_u32)(input).map_err(|e: NomErr<NomErrorr<_>>| ParseError::Nom(e.to_string()))?;
    if payload.len() > 1024 {
        return Err(ParseError::TooLong(payload.len()));
    }

    String::from_utf8(payload.to_vec()).map_err(|e| ParseError::Nom(e.to_string()))
}
