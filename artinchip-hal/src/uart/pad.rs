//! UART pad traits.

pub trait Transmit<const I: u8> {}

pub trait Receive<const I: u8> {}

pub trait IntoTransmit<'a, const I: u8, T> {
    fn into_uart_transmit(self) -> T;
}

pub trait IntoReceive<'a, const I: u8, T> {
    fn into_uart_receive(self) -> T;
}
