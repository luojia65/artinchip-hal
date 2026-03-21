//! UART pad traits.

pub trait Transmit<const I: u8> {}

pub trait Receive<const I: u8> {}
