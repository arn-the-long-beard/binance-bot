use crate::{
    shared::{
        ServerMessage,
        ClientMessage,
    },
    Error,
};
use futures::{
    Stream,
    Sink,
    channel::mpsc::{
        Sender,
        Receiver,
    },
    task::{
        Poll,
        Context,
    },
};
#[allow(unused)]
use tracing::{
    debug,
    error,
};
use std::pin::Pin;

#[derive(Debug)]
pub struct Connection {
    sender: Sender<ClientMessage>,
    receiver: Receiver<ServerMessage>,
}
impl Connection {
    pub fn new(sender: Sender<ClientMessage>, receiver: Receiver<ServerMessage>) -> Self {
        Self {
            sender,
            receiver,
        }
    }
}
impl Stream for Connection {
    type Item = Result<ServerMessage, Error>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        Stream::poll_next(Pin::new(&mut self.receiver), cx)
            .map(|opt| opt.map(Ok))
    }
}
impl Sink<ClientMessage> for Connection {
    type Error = Error;
    fn poll_ready(
        mut self: Pin<&mut Self>,
        cx: &mut Context
    ) -> Poll<Result<(), Self::Error>> {
        Sink::poll_ready(Pin::new(&mut self.sender), cx).map_err(Into::into)
    }
    fn start_send(mut self: Pin<&mut Self>, item: ClientMessage) -> Result<(), Self::Error> {
        Sink::start_send(Pin::new(&mut self.sender), item).map_err(Into::into)
    }
    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut Context
    ) -> Poll<Result<(), Self::Error>> {
        Sink::poll_flush(Pin::new(&mut self.sender), cx).map_err(Into::into)
    }
    fn poll_close(
        mut self: Pin<&mut Self>,
        cx: &mut Context
    ) -> Poll<Result<(), Self::Error>> {
        Sink::poll_close(Pin::new(&mut self.sender), cx).map_err(Into::into)
    }
}
