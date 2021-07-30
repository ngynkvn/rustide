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
pub enum RRequest {
    Kill,
    ImAlive,
    Debug(String),
    State(RustideState),
}

#[derive(Debug)]
pub enum RResponse {
    Ok,
}

#[derive(Debug)]
pub enum RustideMessage {
    Request(RRequest),
    Response(RResponse),
}

impl From<RResponse> for RustideMessage {
    fn from(v: RResponse) -> Self {
        Self::Response(v)
    }
}

impl From<RRequest> for RustideMessage {
    fn from(v: RRequest) -> Self {
        Self::Request(v)
    }
}

impl RustideMessage {
    /// Returns `true` if the rustide_message is [`Response`].
    pub fn is_response(&self) -> bool {
        matches!(self, Self::Response(..))
    }

    /// Returns `true` if the rustide_message is [`Request`].
    pub fn is_request(&self) -> bool {
        matches!(self, Self::Request(..))
    }

    pub fn as_response(&self) -> Option<&RResponse> {
        if let Self::Response(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_request(&self) -> Option<&RRequest> {
        if let Self::Request(v) = self {
            Some(v)
        } else {
            None
        }
    }
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

pub type Endpoint = (Sender<RustideMessage>, Receiver<RustideMessage>);
