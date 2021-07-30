use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, Sender};

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct RustideState {
    pub name: String,
    pub age: u32,
    pub files: Vec<String>,
    pub selection: usize,
}

#[derive(Debug)]
pub enum RustideRequest {
    Kill,
    ImAlive,
    Debug(String),
    State(RustideState)
}

#[derive(Debug)]
pub enum RustideResponse {
    Ok,
}

#[derive(Debug)]
pub enum RustideMessage {
    Request(RustideRequest),
    Response(RustideResponse),
}
pub struct Link {
    e1: (Sender<RustideMessage>, Receiver<RustideMessage>),
    e2: (Sender<RustideMessage>, Receiver<RustideMessage>),
}
impl Link {
    fn new() -> Self {
        Self {
            e1: channel(),
            e2: channel(),
        }
    }
}

pub trait Listen {
    fn listen(self) -> Option<RustideMessage>;
}
pub trait Send {
    fn send<M: Into<RustideMessage>>(&mut self, t: M) -> Option<()>;
}
impl Listen for Endpoint {
    fn listen(self) -> Option<RustideMessage> {
        self.1.try_recv().ok()
    }
}
impl Send for Endpoint {
    fn send<M: Into<RustideMessage>>(&mut self, t: M) -> Option<()> {
        self.0.send(t.into()).ok()
    }
}
impl Into<RustideMessage> for RustideRequest {
    fn into(self) -> RustideMessage {
        RustideMessage::Request(self)
    }
}

pub type Endpoint = (Sender<RustideMessage>, Receiver<RustideMessage>);