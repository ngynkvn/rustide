mod alloc;
mod layout;

#[global_allocator]
pub static ALLOCATOR: alloc::Tracing = alloc::Tracing::new();

use eframe::{
    egui::{self},
    epi,
};

struct Rustide {
    name: String,
    age: u32,
    link: Endpoint,
    debug_strs: Vec<String>,
}

impl Rustide {
    fn new(link: Endpoint) -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            link,
            debug_strs: vec![],
        }
    }
}

impl epi::App for Rustide {
    fn name(&self) -> &str {
        "Rustide"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self {
            link, debug_strs, ..
        } = self;
        if let Ok(msg) = link.1.try_recv() {
            println!("{:?}", msg);
            match msg {
                RustideMessage::Request(RustideRequest::Debug(string)) => {
                    self.debug_strs.push(string);
                }
                RustideMessage::Request(req) => {}
                _ => {}
            }
        }
        //
        // Layout::horizontal(ctx, area, [1, 4]).set_inner(0, |ui| {
        // ui.label("Test") ;
        // });
        //
        //
        egui::SidePanel::left("Files").show(ctx, |ui| {
            for i in &self.debug_strs {
                ui.label(i);
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Text here!");
        });
    }
}

#[derive(Debug)]
pub enum RustideRequest {
    Kill,
    ImAlive,
    Debug(String),
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

trait Listen {
    fn listen(self) -> Option<RustideMessage>;
}
trait Send {
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

type Endpoint = (Sender<RustideMessage>, Receiver<RustideMessage>);

use clap::Clap;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, Sender};

#[derive(Clap, Debug)]
#[clap(name = "rustide")]
struct RustideCli {
    /// Name of the person to greet
    #[clap(short, long, default_value = ".")]
    path: String,
}

fn rustide_backend(path: String, mut channel: Endpoint) -> Result<()> {
    let mark = alloc::Event::Mark;
    eprintln!("{}", serde_json::to_string(&mark).unwrap());
    channel.send(RustideRequest::ImAlive);
    channel.send(RustideRequest::Debug("This is a debug string.".to_string()));
    let read_dir = std::fs::read_dir(path)?;
    for entry in read_dir {
        if let Ok(entry) = entry {
            channel.send(RustideRequest::Debug(
                entry.path().to_string_lossy().to_string(),
            ));
        }
    }
    Ok(())
}

use color_eyre::Result;
use layout::{Layout, LayoutConstraint};
fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = RustideCli::parse();
    let (s1, r1) = channel();
    let (s2, r2) = channel();
    let e1 = (s1, r2);
    let e2 = (s2, r1);
    let options = eframe::NativeOptions::default();
    eprintln!("Running backend in separate thread.");
    crate::ALLOCATOR.set_active(true);
    let rustide_backend = std::thread::spawn(|| rustide_backend(cli.path, e1));
    // eprintln!("Running gui in main loop.");
    eframe::run_native(Box::new(Rustide::new(e2)), options);
}
