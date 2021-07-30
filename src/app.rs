use crate::interop::Endpoint;
use crate::interop::RustideState;
use crate::ui::Chord;
use eframe::{
    egui::{self},
    epi,
};
use std::time::Instant;

trait WidgetAttr {
    const SENSE: egui::Sense;
}

pub struct FileList<'a> {
    list: &'a [String],
}

impl WidgetAttr for FileList<'_> {
    const SENSE: egui::Sense = egui::Sense {
        click: true,
        drag: false,
        focusable: false,
    };
}

impl egui::Widget for FileList<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            for entry in self.list {
                let label = egui::Label::new(entry)
                    .wrap(false)
                    .monospace()
                    .sense(egui::Sense::click());
                let response = ui.add(label);
                if response.clicked() {
                    println!("Clicked {}", entry);
                }
            }
        })
        .response
    }
}
impl<'a> FileList<'a> {
    fn new(list: &'a [String]) -> Self {
        Self { list }
    }
}

pub struct Rustide {
    pub link: Endpoint,
    pub debug_strs: Vec<String>,
    pub state: RustideState,
    pub curr: Instant,
    pub show_explorer: bool,
}
impl Rustide {
    pub fn new(link: Endpoint) -> Self {
        Self {
            link,
            debug_strs: vec![],
            curr: Instant::now(),
            state: RustideState {
                name: "Arthur".to_owned(),
                age: 42,
                files: vec![],
                selection: 0,
            },
            show_explorer: true,
        }
    }
    fn listen(&mut self) {
        use crate::interop::RRequest::*;
        use crate::interop::RustideMessage::Request;
        if let Ok(msg) = self.link.1.try_recv() {
            println!("{:?}", msg);
            match msg {
                Request(Debug(string)) => {
                    self.debug_strs.push(string);
                }
                Request(State(state)) => {
                    self.state = state;
                }
                Request(req) => {}
                _ => {}
            }
        }
    }

    fn handle_input(&mut self, ctx: &egui::CtxRef) {
        let input_state = ctx.input();
        let ctrl_shift_e = Chord::new().ctrl().shift().key(egui::Key::E);
        if input_state.chord_pressed(ctrl_shift_e) {
            self.show_explorer = !self.show_explorer;
        }
    }
}

fn diff(prev: &RustideState, next: &RustideState) {
    unsafe {
        let p: *const RustideState = prev;
        let n: *const RustideState = next;
        let p = p as *const u8;
        let n = n as *const u8;
        let p_slice: &[u8] = std::slice::from_raw_parts(p, std::mem::size_of::<RustideState>());
        let n_slice: &[u8] = std::slice::from_raw_parts(n, std::mem::size_of::<RustideState>());
        for (i, (a, b)) in p_slice.iter().zip(n_slice.iter()).enumerate() {
            if a != b {
                println!("[{}, {}->{}]", i, a, b);
            }
        }
    }
}

pub trait Chords {
    fn chord_pressed(&self, chord: Chord) -> bool;
}

impl Chords for egui::InputState {
    fn chord_pressed(&self, chord: Chord) -> bool {
        chord.matches(self)
    }
}

impl epi::App for Rustide {
    fn name(&self) -> &str {
        "Rustide"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        self.listen();
        self.handle_input(ctx);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        if self.show_explorer {
            egui::SidePanel::left("Explorer").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Time: ");
                    ui.label(format!("{:04}", (Instant::now() - self.curr).as_millis()));
                });
                ui.label(format!("{}", ui.available_width()));
                let file_list = FileList::new(&self.state.files);
                ui.add(file_list);
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.style_ui(ui);
        });
        self.curr = Instant::now();
        ctx.request_repaint();
    }
}
