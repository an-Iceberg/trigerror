#![allow(clippy::needless_return)]
#![allow(mixed_script_confusables)]

pub mod gptp;
pub mod cli;
pub mod trigerror;
pub mod constants;
pub mod packet;
pub mod ring_buffer;
pub mod eth_frame;

/// Extracting the ether type as a u16 number by right shifting the values.
/// [source](https://stackoverflow.com/questions/50243866/how-do-i-convert-two-u8-primitives-into-a-u16-primitive#answer-50244328)
pub fn bytes_to_u16(first_byte: u8, second_byte: u8) -> u16
{ return ((first_byte as u16) << 8) | second_byte as u16; }
