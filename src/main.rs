mod alloc;
mod app;
mod interop;
mod layout;
mod ui;

use crate::app::Rustide;
use crate::interop::RustideState;
use interop::Endpoint;
use interop::Listen;
use interop::RRequest;
use interop::RResponse;
use interop::RustideMessage;
use interop::Send;
use std::sync::mpsc::channel;

// #[global_allocator]
// pub static ALLOCATOR: alloc::Tracing = alloc::Tracing::new();

use clap::Clap;

#[derive(Clap, Debug)]
#[clap(name = "rustide")]
struct RustideCli {
    /// Name of the person to greet
    #[clap(short, long, default_value = ".")]
    path: String,
}

fn rustide_backend(path: String, mut channel: Endpoint) -> Result<()> {
    let mut state = RustideState {
        name: "Kevin".to_string(),
        age: 22,
        files: vec![],
        selection: 0,
    };
    // let mark = alloc::Event::Mark;
    // eprintln!("{}", serde_json::to_string(&mark).unwrap());
    channel.send(RRequest::ImAlive);
    channel.send(RRequest::Debug("This is a debug string.".to_string()));
    let read_dir = std::fs::read_dir(path)?;
    for entry in read_dir {
        if let Ok(entry) = entry {
            state.files.push(entry.path().to_string_lossy().to_string());
        }
    }
    loop {
        channel.send(RRequest::State(state.clone()));
        state.age += 14;
        std::thread::sleep_ms(1000);
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
    // crate::ALLOCATOR.set_active(true);
    let rustide_backend = std::thread::spawn(|| rustide_backend(cli.path, e1));
    // eprintln!("Running gui in main loop.");
    eframe::run_native(Box::new(Rustide::new(e2)), options);
}
