use libc;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Event {
    Alloc { addr: usize, size: usize },
    Mark,
    Freed { addr: usize, size: usize },
}
/// https://fasterthanli.me/articles/small-strings-in-rust
use std::{
    alloc::{GlobalAlloc, System},
    io::Cursor,
    sync::atomic::{AtomicBool, Ordering},
};

pub struct Tracing {
    pub inner: System,
    pub active: AtomicBool,
}

impl Tracing {
    pub const fn new() -> Self {
        Self {
            inner: System,
            active: AtomicBool::new(false),
        }
    }
    fn write_ev(&self, ev: Event) {
        let mut buf = [0u8; 1024];
        let mut cursor = Cursor::new(&mut buf[..]);
        serde_json::to_writer(&mut cursor, &ev).unwrap();
        let end = cursor.position() as usize;
        self.write(&buf[..end]);
        self.write(b"\n");
    }

    pub fn set_active(&self, active: bool) {
        self.active.store(active, Ordering::SeqCst);
    }

    fn write(&self, s: &[u8]) {
        unsafe {
            libc::write(2, s.as_ptr() as _, s.len() as _);
        }
    }
}

unsafe impl GlobalAlloc for Tracing {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        let result = self.inner.alloc(layout);
        if self.active.load(Ordering::SeqCst) {
            self.write_ev(Event::Alloc {
                addr: result as _,
                size: layout.size(),
            });
        }
        result
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: std::alloc::Layout) {
        if self.active.load(Ordering::SeqCst) {
            self.write_ev(Event::Freed {
                addr: ptr as _,
                size: layout.size(),
            });
        }
        self.inner.dealloc(ptr, layout)
    }
}
