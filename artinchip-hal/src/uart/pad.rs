//! UART pad traits.

pub trait Transmit<const I: u8> {}

pub trait Receive<const I: u8> {}

pub trait UartPads<const I: u8> {}

impl<const I: u8, TX, RX> UartPads<I> for (TX, RX)
where
    TX: Transmit<I>,
    RX: Receive<I>,
{
}

pub trait IntoTransmit<'a, const I: u8, T> {
    fn into_uart_transmit(self) -> T;
}

pub trait IntoReceive<'a, const I: u8, T> {
    fn into_uart_receive(self) -> T;
}

pub trait IntoUartPads<'a, const I: u8, T> {
    fn into_uart_pads(self) -> T;
}

impl<'a, const I: u8, T, R, TX, RX> IntoUartPads<'a, I, (TX, RX)> for (T, R)
where
    T: IntoTransmit<'a, I, TX>,
    R: IntoReceive<'a, I, RX>,
{
    #[inline]
    fn into_uart_pads(self) -> (TX, RX) {
        (self.0.into_uart_transmit(), self.1.into_uart_receive())
    }
}
